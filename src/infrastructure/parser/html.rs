use crate::domain::recipes::models::recipe::Recipe;
use crate::domain::recipes::ports::parser::Parser;

use crate::domain::recipes::errors::RecipeError;
use iso8601_duration::Duration;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use serde_json::Value;
use std::error::Error;
use uuid::Uuid;

pub struct SelectParser {}

impl Parser for SelectParser {
    fn parse_recipe(&self, url: String, user_id: String) -> Result<Recipe, Box<dyn Error>> {
        let html = self.get_html(&url)?;
        self.parse_from_json_ld(&url, html.as_str(), user_id)
    }
}

impl SelectParser {
    pub fn new() -> Self {
        SelectParser {}
    }
    fn get_html(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::blocking::get(url)?;
        if response.status() != 200 {
            return Err(Box::new(RecipeError::RecipeNotImported));
        }
        let body = response.text()?;
        Ok(body)
    }

    fn parse_from_json_ld(
        &self,
        url: &str,
        html: &str,
        user_id: String,
    ) -> Result<Recipe, Box<dyn Error>> {
        let document = Document::from(html);
        let mut recipe: Value = document
            .find(Name("script").and(Attr("type", "application/ld+json")))
            .map(|n| n.text())
            .map(|t| {
                let v: Value = serde_json::from_str(&t).unwrap();
                v
            })
            .find(|json| json["@type"] == "Recipe" || json[0]["@type"] == "Recipe")
            .expect("Website not supported");

        if recipe.is_array() {
            recipe = recipe[0].clone();
        }

        let instructions: Vec<String> = recipe["recipeInstructions"]
            .as_array()
            .expect("Impossible to retrieve instructions")
            .iter()
            .map(|i| {
                let text = if i.is_object() { &i["text"] } else { i };
                String::from(
                    text.as_str()
                        .expect("Impossible to retrieve instructions")
                        .trim(),
                )
            })
            .collect();

        let ingredients: Vec<String> = recipe["recipeIngredient"]
            .as_array()
            .expect("Impossible to retrieve ingredients")
            .iter()
            .map(|i| {
                String::from(
                    i.as_str()
                        .expect("Impossible to retrieve ingredients")
                        .trim(),
                )
            })
            .collect();

        let r = Recipe {
            id: Uuid::new_v4(),
            user_id,
            title: self.get_string_field(&recipe["name"]).unwrap(),
            description: self.get_string_field(&recipe["description"]),
            recipe_yield: self.get_string_field(&recipe["recipeYield"]),
            category: self.get_string_field(&recipe["recipeCategory"]),
            cuisine: self.get_string_field(&recipe["recipeCuisine"]),
            prep_time_in_minute: self.get_duration_in_minute(&recipe["prepTime"]),
            cook_time_in_minute: self.get_duration_in_minute(&recipe["cookTime"]),
            instructions,
            ingredients,
            imported_from: Some(String::from(url)),
            image_url: self.get_image(&recipe["image"]),
        };
        Ok(r)
    }

    fn get_duration_in_minute(&self, value: &Value) -> Option<i32> {
        let string = value.as_str()?;
        let duration = Duration::parse(string).ok()?;
        Some((duration.minute as i32) + (duration.hour as i32 * 60))
    }

    fn get_string_field(&self, value: &Value) -> Option<String> {
        if value.is_f64() {
            value.as_f64().map(|n| n.to_string())
        } else if value.is_i64() {
            value.as_i64().map(|n| n.to_string())
        } else {
            value
                .as_str()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(String::from)
        }
    }

    fn get_image(&self, value: &Value) -> Option<String> {
        if value.is_object() {
            self.get_string_field(&value["url"])
        } else {
            self.get_string_field(value)
        }
    }
}

impl Default for SelectParser {
    fn default() -> Self {
        SelectParser::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parsing_marmitton() {
        // given
        let html = fs::read_to_string("./src/infrastructure/parser/__data__/marmitton.html")
            .expect("Something went wrong reading the file");
        let parser = SelectParser::new();
        let user_id = String::from("some_user_id");
        let url = "https://www.marmiton.org/recettes/recette_pate-brisee-vite-faite_31639.aspx";

        // when
        let recipe = parser
            .parse_from_json_ld(url, html.as_str(), user_id.clone())
            .expect("Can parse recipe");

        // then
        assert_eq!(recipe.user_id, user_id);
        assert_eq!(recipe.imported_from, Some(String::from(url)));
        assert_eq!(recipe.title, "Pâte brisée vite faite");
        assert_eq!(recipe.prep_time_in_minute, Some(15));
        assert_eq!(recipe.cook_time_in_minute, Some(30));
        assert_eq!(recipe.instructions, vec![
            String::from("Mélanger la farine et le sel dans un plat (et le sucre si sucre il y a)."),
            String::from("Ajouter le beurre puis l\'incorporer à la farine en pétrissant rapidement et légèrement du bout des doigts. On doit obtenir une sorte de semoule grossière en 2 ou 3 min."),
            String::from("Incorporer rapidement le lait ou l\'eau. Il en faut très peu pour permettre à la pâte de se lier et de faire boule. Le lait ou l\'eau ? C\'est selon les goûts."),
            String::from("Pour étaler sans problème, étaler la pâte sur du papier cuisson et cuire tel quel dans la platine.")
            ]
        );
        assert_eq!(
            recipe.ingredients,
            vec![
                String::from("300 g de farine"),
                String::from("150 g de beurre en dés et en pommade"),
                String::from("1/2 cuillère à café de sel"),
                String::from("3 cuillères à soupe de sucre (si pâte sucrée)"),
                String::from("8 cl d\'eau ou de lait tiède")
            ]
        );
        assert_eq!(recipe.category, Some(String::from("pâte à tarte salée")));
        assert_eq!(
            recipe.description,
            Some(String::from("farine, beurre, sel, sucre, eau"))
        );
        assert_eq!(
            recipe.image_url,
            Some(String::from(
                "https://assets.afcdn.com/recipe/20160331/12788_w1024h768c1cx983cy1500.jpg"
            ))
        );
        assert_eq!(recipe.cuisine, None);
        assert_eq!(recipe.recipe_yield, Some(String::from("1 pâte")));
    }

    #[test]
    fn parsing_journal_des_femmes() {
        // given
        let html =
            fs::read_to_string("./src/infrastructure/parser/__data__/journal_des_femmes.html")
                .expect("Something went wrong reading the file");
        let parser = SelectParser::new();
        let user_id = String::from("some_user_id");
        let url = "https://cuisine.journaldesfemmes.fr/recette/313738-lasagnes-a-la-bolognaise";

        // when
        let recipe = parser
            .parse_from_json_ld(url, html.as_str(), user_id.clone())
            .expect("Can parse recipe");

        // then
        assert_eq!(recipe.user_id, user_id);
        assert_eq!(recipe.imported_from, Some(String::from(url)));
        assert_eq!(
            recipe.title,
            String::from("Lasagnes : la meilleure recette")
        );
    }

    #[test]
    fn parsing_taste_com_au() {
        // given
        let html = fs::read_to_string("./src/infrastructure/parser/__data__/taste_com_au.html")
            .expect("Something went wrong reading the file");
        let parser = SelectParser::new();
        let user_id = String::from("some_user_id");
        let url = "https://www.taste.com.au/recipes/better-you-chicken-cacciatore/47u4vq3q";

        // when
        let recipe = parser
            .parse_from_json_ld(url, html.as_str(), user_id.clone())
            .expect("Can parse recipe");

        // then
        assert_eq!(recipe.user_id, user_id);
        assert_eq!(recipe.imported_from, Some(String::from(url)));
        assert_eq!(
            recipe.title,
            String::from("Better-for-you chicken cacciatore")
        );
        assert_eq!(recipe.recipe_yield, Some(String::from("6")));
        assert_eq!(recipe.image_url, Some(String::from("https://img.taste.com.au/UBlf4nO-/taste/2018/05/better-for-you-chicken-cacciatore-137669-2.jpg")));
    }
}

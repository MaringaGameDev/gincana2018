use ::rocket_contrib::{Json, Template};
use ::serde::ser::{Serialize, Serializer};

enum CategoryEnum { General, Sports, Entertainment, Science, Present, }

impl Serialize for CategoryEnum {
   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: Serializer {
      match *self {
         CategoryEnum::General       => serializer.serialize_str("general"),
         CategoryEnum::Sports        => serializer.serialize_str("sports"),
         CategoryEnum::Entertainment => serializer.serialize_str("entertainment"),
         CategoryEnum::Science       => serializer.serialize_str("science"),
         CategoryEnum::Present       => serializer.serialize_str("present"),
      }
   }
}

enum DifficultyEnum { Potato, Easy, Regular, Medium, Hard, Extreme, }

impl Serialize for DifficultyEnum {
   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: Serializer {
      match *self {
         DifficultyEnum::Potato  => serializer.serialize_str("potato"),
         DifficultyEnum::Easy    => serializer.serialize_str("easy"),
         DifficultyEnum::Regular => serializer.serialize_str("regular"),
         DifficultyEnum::Medium  => serializer.serialize_str("medium"),
         DifficultyEnum::Hard    => serializer.serialize_str("hard"),
         DifficultyEnum::Extreme => serializer.serialize_str("extreme"),
      }
   }
}

#[derive(Serialize)]
struct QuestionItem {
   diff: DifficultyEnum,
   title: String,
   answer: String,
}

#[derive(Serialize)]
struct CategoryItem {
   name: CategoryEnum,
   questions: [QuestionItem; 6],
}

#[derive(Serialize)]
struct QuizContext {
   quiz: [CategoryItem; 5],
}

#[get("/quiz")]
fn quiz() -> Template {
   let context = QuizContext {
      quiz: [
         CategoryItem {
            name: CategoryEnum::General,
            questions: [
               QuestionItem {
                  diff: DifficultyEnum::Potato,
                  title: "A".to_string(),
                  answer: "B".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Easy,
                  title: "C".to_string(),
                  answer: "D".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Regular,
                  title: "E".to_string(),
                  answer: "F".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Medium,
                  title: "G".to_string(),
                  answer: "H".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Hard,
                  title: "I".to_string(),
                  answer: "J".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Extreme,
                  title: "K".to_string(),
                  answer: "L".to_string(),
               },
            ],
         },
         CategoryItem {
            name: CategoryEnum::Sports,
            questions: [
               QuestionItem {
                  diff: DifficultyEnum::Potato,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Easy,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Regular,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Medium,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Hard,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Extreme,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
            ],
         },
         CategoryItem {
            name: CategoryEnum::Entertainment,
            questions: [
               QuestionItem {
                  diff: DifficultyEnum::Potato,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Easy,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Regular,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Medium,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Hard,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Extreme,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
            ],
         },
         CategoryItem {
            name: CategoryEnum::Science,
            questions: [
               QuestionItem {
                  diff: DifficultyEnum::Potato,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Easy,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Regular,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Medium,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Hard,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Extreme,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
            ],
         },
         CategoryItem {
            name: CategoryEnum::Present,
            questions: [
               QuestionItem {
                  diff: DifficultyEnum::Potato,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Easy,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Regular,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Medium,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Hard,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
               QuestionItem {
                  diff: DifficultyEnum::Extreme,
                  title: "".to_string(),
                  answer: "".to_string(),
               },
            ],
         },
      ],
   };
   Template::render("quiz", &context)
}

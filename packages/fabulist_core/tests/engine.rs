use fabulist_core::{
    engine::Engine,
    state::State,
    story::{
        character::Character,
        choice::ChoiceBuilder,
        context::ContextValue,
        dialogue::{DialogueBuilder, DialogueLayout},
        part::PartBuilder,
        StoryBuilder,
    },
};

#[test]
pub fn engine_tests() {
    let story = StoryBuilder::new()
        .set_start("start-scene")
        .add_character(Character::new("dave", "Dave"))
        .add_part(
            PartBuilder::new("start-scene")
                .add_dialogue(
                    DialogueBuilder::new(DialogueLayout {
                        text: "Hi, there!".to_string(),
                        character: "dave".to_string(),
                    })
                    .build(),
                )
                .add_dialogue(
                    DialogueBuilder::new(DialogueLayout {
                        text: "What's your favorite fruit?".to_string(),
                        character: "dave".to_string(),
                    })
                    .add_choice(
                        ChoiceBuilder::new("Apple")
                            .set_change_context(|context| {
                                context.0.insert(
                                    "favorite_fruit".to_string(),
                                    ContextValue::String("Apple".to_string()),
                                );
                            })
                            .build(),
                    )
                    .add_choice(
                        ChoiceBuilder::new("Banana")
                            .set_change_context(|context| {
                                context.0.insert(
                                    "favorite_fruit".to_string(),
                                    ContextValue::String("Banana".to_string()),
                                );
                            })
                            .build(),
                    )
                    .build(),
                )
                .add_dialogue(
                    DialogueBuilder::new(DialogueLayout {
                        text: "Oh cool! But that's a bit...".to_string(),
                        character: "dave".to_string(),
                    })
                    .set_next(|context| {
                        let favorite_fruit = context
                            .0
                            .get("favorite_fruit")
                            .expect("Context to have a `favorite_fruit` property");
                        let favorite_fruit = match favorite_fruit {
                            ContextValue::String(string_content) => string_content,
                            _ => panic!("Context value `favorite_fruit` was not a string."),
                        };
                        if favorite_fruit == "Apple" {
                            "fail-scene".to_string()
                        } else {
                            "success-scene".to_string()
                        }
                    })
                    .build(),
                )
                .build(),
        )
        .add_part(
            PartBuilder::new("fail-scene")
                .add_dialogue(
                    DialogueBuilder::new(DialogueLayout {
                        text: "Oh. That's pretty generic.".to_string(),
                        character: "dave".to_string(),
                    })
                    .build(),
                )
                .build(),
        )
        .add_part(
            PartBuilder::new("success-scene")
                .add_dialogue(
                    DialogueBuilder::new(DialogueLayout {
                        text: "Me too!".to_string(),
                        character: "dave".to_string(),
                    })
                    .build(),
                )
                .build(),
        )
        .build();

    let mut engine = Engine::new(story, State::new());

    let result = engine.start().unwrap();
    assert_eq!(result.part_key, "start-scene");
    assert_eq!(result.dialogue_index, 0);

    let result = engine.next(None).unwrap();
    assert_eq!(result.part_key, "start-scene");
    assert_eq!(result.dialogue_index, 1);

    let result = engine.next(Some(0)).unwrap();
    assert_eq!(result.part_key, "start-scene");
    assert_eq!(result.dialogue_index, 2);

    let result = engine.next(None).unwrap();
    assert_eq!(result.part_key, "fail-scene");
    assert_eq!(result.dialogue_index, 0);

    let result = engine.next(None);
    assert!(result.is_err());
}

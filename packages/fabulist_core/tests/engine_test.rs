use fabulist_core::{
    engine::Engine,
    state::State,
    story::{
        character::CharacterBuilder,
        choice::ChoiceBuilder,
        context::ContextValue,
        dialogue::{DialogueBuilder, DialogueLayout},
        part::PartBuilder,
        StoryBuilder,
    },
};

#[test]
pub fn engine_runs_basic_story() {
    let story = StoryBuilder::new()
        .set_start("start-scene")
        .add_character(
            CharacterBuilder::new("dave", "Dave")
                .set_long("Daveren Cordero")
                .set_nick("Core"),
        )
        .add_part(
            PartBuilder::new("start-scene")
                .add_dialogue(DialogueBuilder::new(DialogueLayout {
                    text: "Hi, there!",
                    character: "dave",
                }))
                .add_dialogue(
                    DialogueBuilder::new(DialogueLayout {
                        text: "What's your favorite fruit?",
                        character: "dave",
                    })
                    .add_choice(ChoiceBuilder::new("Apple").set_change_context(|context| {
                        context.insert("favorite_fruit", "Apple");
                    }))
                    .add_choice(
                        ChoiceBuilder::new("Banana").set_change_context(|context| {
                            context.insert("favorite_fruit", "Banana");
                        }),
                    ),
                )
                .add_dialogue(
                    DialogueBuilder::new(DialogueLayout {
                        text: "Oh cool! But that's a bit...",
                        character: "dave",
                    })
                    .set_query_next(|context| {
                        let favorite_fruit = context
                            .value()
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
                    }),
                ),
        )
        .add_part(
            PartBuilder::new("fail-scene").add_dialogue(DialogueBuilder::new(DialogueLayout {
                text: "Oh. That's pretty generic.",
                character: "dave",
            })),
        )
        .add_part(
            PartBuilder::new("success-scene").add_dialogue(DialogueBuilder::new(DialogueLayout {
                text: "Me too!",
                character: "dave",
            })),
        )
        .build();

    let mut state = State::new();
    let mut engine = Engine::new(story, &mut state);

    let result = engine.start().unwrap();
    assert_eq!(result.part_key, "start-scene".into());
    assert_eq!(result.dialogue_index, 0);

    let result = engine.next(None).unwrap();
    assert_eq!(result.part_key, "start-scene".into());
    assert_eq!(result.dialogue_index, 1);

    let result = engine.next(Some(0)).unwrap();
    assert_eq!(result.part_key, "start-scene".into());
    assert_eq!(result.dialogue_index, 2);

    let result = engine.next(None).unwrap();
    assert_eq!(result.part_key, "fail-scene".into());
    assert_eq!(result.dialogue_index, 0);

    let result = engine.next(None);
    assert!(result.is_err());
}

# Prototype Spec

## Simple Dialogue

```plaintext
# "dialogue-1"
[Jose]
> "What's up?"
    - "The ceiling." => {
        next: (context) => {},
        change_context: (context) => {},
    }
    - "Nothing much." => {
        next: (context) => {},
        change_context: (context) => {},
    }
```

## Simple Monologue

```plaintext
# "monologue-1"
[Jose]
> "What's up?"
> "Doing good?"
> "Why aren't you responding?"
    - "Wait, you were talking?" 
    - "Sorry, I was distracted."
```

## Simple Part

```plaintext
# "dialogue-3" => {
    background: "./bg/starry_night.png"
}
[Jose]
> "When are you getting a car?"
[Dave]
> "Right, I was wondering about that as well."
    - "I'm flat out broke man." 
    - "In a few years. I just need to sell more coccaine. "
```

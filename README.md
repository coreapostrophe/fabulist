![Fabulist core banner](./assets/readme-banner.png)

## Fabulist Core
![Build](https://github.com/daverencordero/fabulist-core/actions/workflows/build.yml/badge.svg)

### About
This is a builder that uses a context-based decision tree model to create a branching narrative. A plethora of the currently existing API interfaces used for designing novels with user interactions *(e.g. Visual Novels or Text-based adventures)* are plagued with complexity and a steep learning curve. Due to this, the unique art medium is hardly used and is driven to obscurity. Fabulist aims to manifest the potential of branching narratives by introducing a new standard and making their creation facile to the masses.

### Contents
The library exports the main story structure as well as the structures made to build it. This allows developers to immediately integrate or iterate the concept immediately. In addition, an engine that uses the structure to create a story runtime is also included for debugging as well as future projects related to Fabulist.

### Roadmap
The core will serve as a foundation of an ecosystem of applications planned to bring fabulist to everyone. As such, it is tantamount that it's entirely flexible and has all the utilities needed to create a narrative. Future iterations of this library aims to have transpilation to widely used languages (JavaScript, Java, and Python) as well as a markup language preprocessor for industry-use.

### Design
The fabulist story model revolves around a tree structure. Each node of the tree can either be a `dialogue` or a `part`.
```
└── story node/
    ├── dialogue
    └── part
```
1. **Dialogue** - the dialogue type is where most of the important data is stored. Things such as the conversant's name, what they said, and the response options, they're all found inside the dialogue structure.
2. **Part** - the part type is simply a container that groups dialogues together. This introduces the concept of story partitions such as Scenes, Parts, Acts, or Chapters.

A fabulist story is basically a collection of these two types of nodes. But you're probably wondering, **how are they linked together?** Traditional branching narrative methods made it so that each response would have a corresponding dialogue linked to it. While this is effective, it is far from being optimal and usually results into little flexibility for future changes. This is why other API interfaces compensate for the lack of flexibility with a bloat of functions to fill the gap.

Fabulist is different.

Instead of a direct link, fabulist uses context. **Context** is a metadata attached to the story that all the nodes can access. The context is what dictates the flow of the narrative as well as the linkages of the nodes. It can be any form of data that can be used as a condition. For example and clarification, refer to the pseudocode below.

```
context = { are_friends = true }

dialogue_1 = {
    get_next () {
        if (context.are_friends = true) return dialogue_2
        else return dialogue_3
    }
}
```

In the statement, we defined a context with a property named `are_friends` that has a value of `true`. We then used this context as a reference upon defining `dialogue_1`. In its `get_next()` function, we have a condition that tests whether the `are_friends` property is true or not. Depending on the result, different dialogues are returned. On implementation, the `get_next()` function would be called upon progressing through the dialogue.

This type of linking facilitates a more natural way of creating a narrative. Instead of linking nodes to a response, we instead base it on a value we stored from a context. 

But how do we alter the context? The Context will be altered upon entering a new story node. Each story node could have a `change_context()` function that fires whenever it is in the active progression of the narrative. See the example below.

```
context = { are_friends = false }

dialogue {
    change_context() {
        context.are_friends = true
    }
}
```

The application for this is limitless! Using dating simulators as an example (Only because they're a predominant part of the Visual Novel space), affection points of potential lovers could be stored inside the context as incrementing values. This would mean that each dialogue could update the context whence necessary resulting in a much more organic flow! 

And, that's it. Nothing more but an incredibly simple, scalable, and approachable interface.

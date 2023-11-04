![Fabulist banner](./assets/readme-banner.png)

## ✒️ Fabulist

### About
Fabulist is a builder that uses a context-based model to create a branching narrative. A plethora of the currently existing API interfaces used for designing interactive novels *(e.g. Visual Novels or Text-based adventures)* are plagued with complexity and a steep learning curve. I believe that it's due to this inaccessibility that drives the unique art medium to obscurity. This library aims to manifest the potential of branching narratives by introducing a new standard and making their creation approachable to the masses.

### Contents
1. `fabulist_core` - The structural crate of the project. It contains the base specification of the library as well as the data structures that concerns it. 

### Roadmap
The core will serve as the foundation of an ecosystem of applications planned to bring fabulist to everyone. As such, it is tantamount that it's entirely flexible and has all the utilities needed to create a narrative. Future iterations of this library aims to have transpilation to widely used languages (JavaScript, Java, and Python) as well as a markup language preprocessor for industry-use.

```mermaid
timeline
    February: Fabulist Core
    October: Refactored Fabulist Core
    December: Fabulist compiler
```

### Design

#### Structure
The fabulist story model revolves around a loose tree structure. Each node of the tree can either be a `dialogue` or a `part`.

```mermaid
classDiagram
    StoryNode --|> Part
    StoryNode --|> Dialogue
    class StoryNode {
        +"Part"|"Dialogue" node_type
    }
    class Part{
        +StoryNode[] story_nodes
    }
    class Dialogue{
        +String speaker
        +Quote[] quotes
    }
```
1. **Dialogue** - the dialogue type is where most of the important information is stored. Things such as the speaker's name, what they said are all found inside the dialogue structure.
> Note: **quotes** are the responses of the speakers. If the dialogue node only has one quote, then that suggests a linear progression. This means that the speaker simply said such quote. Multiple quotes, on the other hand, suggests that a decision of what the speaker should say is necessary. This is where branches are made from the narrative. 
2. **Part** - the part type is simply a container that groups other story nodes together. This introduces the concept of story partitions such as Scenes, Parts, Acts, or Chapters.

An entire fabulist story is simply a collection of these two node types. As you can see, the structure is easily digestible or comprehensive.

#### Linkages

Instead of one-to-one correspondence between a response (common to traditional implementations), fabulist uses something called the **Context**. 

The story context is a metadata attached to the story that is globally accessible to all the nodes. It is what dictates the flow of the narrative and is why I refer to the story structure as *loose*. 

It's easier to explain its use from an example. Let's say we have a value named `are_friends` within our story's context.
```mermaid
classDiagram
    class Context {
        +bool are_friends
    }
```

We can mutate that value with the dialogue's `change_context()`.
```mermaid
classDiagram
    class Dialogue{
        +String speaker
        +Quote[] quotes
        +change_context()
        +next()
    }
```

The flow should look something like this.
```mermaid
flowchart LR
    dialogue_1[Dialogue 1] --o dialogue_1_on_next

    dialogue_1_on_next("`
    change_context(context) {
        context.are_friends = true;
        return context;
    }
    `")
    style dialogue_1_on_next text-align:left

    dialogue_1 --o dialogue_1_on_change

    dialogue_1_on_change{`are_friends` is true} -->|Yes| dialogue_2
    dialogue_1_on_change -->|No| dialogue_3
    style dialogue_1_on_change text-align:left

    dialogue_2[Dialogue 2]
    dialogue_3[Dialogue 3]
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

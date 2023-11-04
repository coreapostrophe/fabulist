### Prototype Specification

#### Comments

<u>syntax:</u>
```
// <comment message>

```

<u>example:</u>
```
// I am an example comment
```

#### Simple Conversation

<u>syntax:</u>
```
[<node_id: string | u32>]
<speaker>:
<quote>
```

<u>example:</u>
```
[01]
Dave:
Hello!

[02]
Core:
Hi!
```

#### Dialogue w/ metadata

<u>syntax:</u>
```
[<node_id: string | u32>]
--- 
<key>: <value>
..
---
<speaker>:
<quote>

```

<u>example:</u>
```
[01]
--- 
background: 'path/to/file';
---
Dave:
Hello!
```

#### Explicit next node

<u>syntax:</u>
```
[<node_id: string | u32>]
--- 
<key>: <value>
..
---
<speaker>:
<quote> => <node id>

```

<u>example:</u>
```
[01]
Dave:
Hello! => 02
```

#### Next node function

<u>syntax:</u>
```
[<node_id: string | u32>]
--- 
<key>: <value>
..
---
<speaker>:
<quote> => {
    // any expression/mutation of context
    return <node id>
}
```

<u>example:</u>
```
[01]
Dave:
Hello! => {
    has_greeted = true
    return 02
}
```

#### Next node function

<u>syntax:</u>
```
[<node_id: string | u32>]
--- 
<key>: <value>
..
---
<speaker>:
<quote> => /path/to/file
```

<u>example:</u>
```
[01]
Dave:
Hello! => /file.type
```
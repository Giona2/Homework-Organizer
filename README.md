# Table of Contents
* [Description](#description)
* [Example Usage](#example-usage)
* [Commands](#commands)

# Description
`homework_organizer` is a very simple to-do TUI application meant to keep track of tasks that need to be completed by organizing them into groups  
  
This application was developed for a blazingly fast workflow. This is supported by features such as each command only being two letters long, simple command-line parsing, and each command taking very few arguments  
  
*Note that only Unix-based operating systems are supported*

# Example Usage
On startup you should see a basic interface similar to the following
```bash

> 
```
  
---
  
To create a new group, Algebra class for example, type `ac` followed by the name of the class and its tag identifier then hit enter  
  
*Tags are a way to simplify the editing of class groups. When you want to edit a group, you only have to type its tag rather than the entire name of the class or its index. They can be any length*  
```bash

> ac Algebra a
```
  
---
  
You should then see the class appear in the interface. The identifier is printed in brackets, then the name of the class is displayed next to it
```bash

[a] Algebra:

>
```
  
---
  
Let's add another class. English, for example.
```bash

[a] Algebra

> ac English e
```
```bash

[a] Algebra:

[e] English:

>
```
  
---
  
`homework_organizer` adds classes chronologically. If you want to move a classes position up or down, simply type `mc` followed by the tag, then type `u` to move it up or `d` to move it down  
  
*For the techier ones out there, the table is serialized into an IndexMap, allowing each class to also have an index rather than just a key*
```bash

[a] Algebra:

[e] English:

> mc e u
```
```bash

[e] English:

[a] Algebra:

>
```
  
---
  
Once you have the order of your classes right you should then add assignments you have to complete. You can do this by inserting the `aa` command, the class tag, and the name of the assignment  
  
*For simplicity, the name of the assignment is interpretted as all the content after the class tag. IE, you do not have to place the assignment's name between quotes for it to register. This will actually result in undesired behavior*
```bash

[e] English:

[a] Algebra:

> aa a Math homework
```
```bash

[e] English:

[a] Algebra:
    1. Math homework

>
```
  
---
  
And when you're finished with the assignment, remove it with the `ra` command by specifying the classes tag and index of the assignment  
  
*The indexing starts at `1`*
```bash

[e] English:

[a] Algebra:
    1. Math homework

> ra a 1
```
```bash

[e] English:

[a] Algebra:

>
```
  
---
  
Finally, when you're done for the day, exit the program either by typing `e` or `exit`
```bash

[e] English:

[a] Algebra:

> e
```
  
---
  
For more commands, type `h` or `help` for the help message. See this help message here by seeing the [Commands](#commands) section

# Commands
```bash
ac <class_name> <class_tag>
    Add a class

rc <class_tag>
    Remove a class

cc <class_tag> <new_class_tag>
    Change a classes tag

mc <class_tag> <class_position>
    Move a class up [u] or down [d]

aa <class_tag> <assignment>
    Add an assignment to a class

ra <class_tag> <assignment_index>
    Remove an assignment from a class

ca <class_tag>
    Clear a classes assignments
```

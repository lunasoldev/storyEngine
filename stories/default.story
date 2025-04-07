[[scene]]
id = "end_good"
text = "You survived and escaped the building! You're free!"
choices = [
  { text = "the end", target = "end" }
]

[[scene]]
id = "end_bad"
text = "You jump out of the window, you die."
choices = [
  { text = "the end", target = "end" }
]

[[scene]]
id = "start"
text = "You wake up in a strange place. The air smells off."
choices = [
    { text = "Look around", target = "look_around" },
    { text = "Stay still", target = "debug" }
]

[[scene]]
id = "look_around"
text = "You see a door and a window."
choices = [
    { text = "Approach the door", target = "end_good" },
    { text = "Approach the window", target = "end_bad" }
]

[[scene]]
id = "debug"
text = "This branch has not been created yet!"
choices = [
    { text = "bruh", target = "end" }
]

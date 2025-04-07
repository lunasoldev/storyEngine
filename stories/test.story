[[scene]]
id = "start"
text = "This is a debug story file. It is used to test storyWhateverIcalledIt during development...\nYou have the following dev tools:"
choices = [
    { text = "Test variables", target = "var_test" }
]

[[scene]]
id = "var_test"
text = "this is a test path to test variables"
choices = [
  { text = "create debugval2", target = "var_test", action = { variable = "debugval2", action_type = "set", value = 0 } },
  { text = "add 1 to debugval2", target = "var_test", action = { variable = "debugval2", action_type = "add", value = 1 } },
  { text = "sub 1 from debugval2", target = "var_test", action = { variable = "debugval2", action_type = "subtract", value = 1 } },

  { text = "create debugval1", target = "var_test", action = { variable = "debugval1", action_type = "set", value = 0 } },
  { text = "add 1 to debugval1", target = "var_test", action = { variable = "debugval1", action_type = "add", value = 1 } },
  { text = "sub 1 from debugval1", target = "var_test", action = { variable = "debugval1", action_type = "subtract", value = 1 } },
  { text = "close", target = "end" }
]

--!nocheck
local result = require("fake_script")

print(result)

assert(typeof(result) == "number")
assert(result == 2000)

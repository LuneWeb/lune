--!nocheck
local result = require("@custom/number")

print(result)

assert(typeof(result) == "number")
assert(result == 6009)

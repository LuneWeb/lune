local roblox = require("@lune/roblox") :: any
local Vector2 = roblox.Vector2

-- Constructors & properties

Vector2.new()
Vector2.new(0)
Vector2.new(0, 0)
Vector2.new(0 / 0, 0 / 0)

assert(not pcall(function()
	return Vector2.new(false)
end))
assert(not pcall(function()
	return Vector2.new("", "")
end))
assert(not pcall(function()
	return Vector2.new(newproxy(true))
end))

assert(Vector2.new(1, 2).X == 1)
assert(Vector2.new(1, 2).Y == 2)

-- Constants

assert(Vector2.one == Vector2.new(1, 1))
assert(Vector2.zero == Vector2.new(0, 0))
assert(Vector2.xAxis == Vector2.new(1, 0))
assert(Vector2.yAxis == Vector2.new(0, 1))
assert(Vector2.zAxis == nil)

-- Ops

assert(Vector2.new(2, 4) + Vector2.new(1, 1) == Vector2.new(3, 5))
assert(Vector2.new(2, 4) - Vector2.new(1, 1) == Vector2.new(1, 3))
assert(Vector2.new(2, 4) * Vector2.new(1, 2) == Vector2.new(2, 8))
assert(Vector2.new(2, 4) / Vector2.new(1, 2) == Vector2.new(2, 2))

assert(Vector2.new(2, 4) * 2 == Vector2.new(4, 8))
assert(Vector2.new(2, 4) / 2 == Vector2.new(1, 2))

assert(Vector2.new(7, 15) // Vector2.new(3, 7) == Vector2.new(2, 2))
assert(Vector2.new(3, 7) // 2 == Vector2.new(1, 3))

-- Vector math methods
assert(Vector2.new(-1, -2):Abs() == Vector2.new(1, 2))
assert(Vector2.new(-1.7, 2):Sign() == Vector2.new(-1, 1))
assert(Vector2.new(-1.9, 2.1):Ceil() == Vector2.new(-1, 3))
assert(Vector2.new(-1.1, 2.99):Floor() == Vector2.new(-2, 2))

assert(Vector2.new(1, 2):FuzzyEq(Vector2.new(1 - 1e-6, 2 + 1e-6), 1e-5))
assert(not Vector2.new(1, 2):FuzzyEq(Vector2.new(1.2, 2), 0.1))

local angle = Vector2.new(1, 1):Angle(Vector2.new(-1, 1))
assert(math.abs(angle - (math.pi / 2)) < 1e-5)

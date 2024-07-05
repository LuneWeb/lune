local function getW()
	return string.rep("w", 999)
end

return {
	getW = getW,
	someLongFunction = function()
		print(debug.info(1, "n"), debug.info(2, "n"), debug.info(3, "n"))
		print(debug.info(1, "n"), debug.info(2, "n"), debug.info(3, "n"))
	end,
}

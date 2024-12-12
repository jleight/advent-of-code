namespace AoC.Contexts;

public record SolutionContext(
    string InputString,
    string[] InputLines)
{
    public Grid<char> ToGrid(
        char defaultValue,
        params HashSet<char> ignore)
    {
        var grid = Grid.New(
            InputLines[0].Length,
            InputLines.Length,
            defaultValue);

        ignore.Add(defaultValue);

        for (var y = 0; y < grid.Height; y++)
        for (var x = 0; x < grid.Width; x++)
        {
            var c = InputLines[y][x];

            if (ignore.Contains(c))
                continue;

            grid[x, y] = c;
        }

        return grid;
    }
}

namespace AoC.Y2023.Solutions;

public class D11P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var grid = Grid.New(
            context.InputLines[0].Length,
            context.InputLines.Length,
            '.');

        var galaxies = 0;

        for (var y = 0; y < context.InputLines.Length; y++)
        for (var x = 0; x < context.InputLines[y].Length; x++)
            if (context.InputLines[y][x] == '#')
                grid[x, y] = (++galaxies).ToString()[0];

        var xs = grid
            .Select((point, _) => point.X)
            .ToHashSet();
        var ys = grid
            .Select((point, _) => point.Y)
            .ToHashSet();

        var missingXs = Enumerable
            .Range(0, grid.Width)
            .Except(xs)
            .ToArray();
        var missingYs = Enumerable
            .Range(0, grid.Height)
            .Except(ys)
            .ToArray();

        var sum = 0;

        foreach (var (a, b, _) in grid)
        {
            foreach (var (c, d, _) in grid)
            {
                if (a == c && b == d)
                    continue;

                var minX = Math.Min(a, c);
                var maxX = Math.Max(a, c);
                var minY = Math.Min(b, d);
                var maxY = Math.Max(b, d);

                var dX = missingXs.Count(x => x > minX && x < maxX);
                var dY = missingYs.Count(y => y > minY && y < maxY);

                var distance = maxX - minX + dX +
                               maxY - minY + dY;

                sum += distance;
            }
        }

        Console.WriteLine(sum / 2);
        return Task.CompletedTask;
    }
}

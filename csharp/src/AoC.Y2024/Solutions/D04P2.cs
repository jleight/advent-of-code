namespace AoC.Y2024.Solutions;

public class D04P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var grid = Grid.New(
            context.InputLines[0].Length,
            context.InputLines.Length,
            ' ');

        for (var y = 0; y < context.InputLines.Length; y++)
        for (var x = 0; x < context.InputLines[y].Length; x++)
            if (context.InputLines[y][x] is 'M' or 'A' or 'S')
                grid[x, y] = context.InputLines[y][x];

        var states = grid
            .Where(c => c.Cell == 'M')
            .SelectMany(c => Enum
                .GetValues<Direction>()
                .Select(d => new Match(c.X, c.Y, d, 1)))
            .ToList();
        var nextStates = new List<Match>();

        var centers = new HashSet<Center>();

        while (states.Count > 0)
        {
            foreach (var state in states)
            {
                var (fromX, fromY, direction, found) = state;

                if (found == 3)
                {
                    var (ax, ay) = direction switch
                    {
                        Direction.Ne => (fromX + 1, fromY - 1),
                        Direction.Se => (fromX + 1, fromY + 1),
                        Direction.Sw => (fromX - 1, fromY + 1),
                        Direction.Nw => (fromX - 1, fromY - 1),
                        _ => throw new ArgumentOutOfRangeException(nameof(direction), direction, null)
                    };

                    centers.Add(new(ax, ay, direction));
                    continue;
                }

                var (x, y) = direction switch
                {
                    Direction.Ne => (fromX + found, fromY - found),
                    Direction.Se => (fromX + found, fromY + found),
                    Direction.Sw => (fromX - found, fromY + found),
                    Direction.Nw => (fromX - found, fromY - found),
                    _ => throw new ArgumentOutOfRangeException(nameof(direction), direction, null)
                };

                if (x < 0 || x >= grid.Width || y < 0 || y >= grid.Height)
                    continue;

                var next = found switch
                {
                    1 => 'A',
                    2 => 'S',
                    _ => throw new InvalidOperationException("Already done!")
                };

                if (grid[x, y] != next)
                    continue;

                nextStates.Add(new(fromX, fromY, direction, found + 1));
            }

            states = nextStates;
            nextStates = new();
        }

        var total = centers
            .GroupBy(c => (c.X, c.Y))
            .Count(g => g.Count() == 2);

        Console.WriteLine(total);
        return Task.CompletedTask;
    }
}

file sealed record Match(
    int X,
    int Y,
    Direction Direction,
    int Found);

file sealed record Center(
    int X,
    int Y,
    Direction Direction);


file enum Direction
{
    Ne,
    Se,
    Sw,
    Nw
}

namespace AoC.Y2024.Solutions;

public class D04P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var grid = Grid.New(
            context.InputLines[0].Length,
            context.InputLines.Length,
            ' ');

        for (var y = 0; y < context.InputLines.Length; y++)
        for (var x = 0; x < context.InputLines[y].Length; x++)
            if (context.InputLines[y][x] is 'X' or 'M' or 'A' or 'S')
                grid[x, y] = context.InputLines[y][x];

        var states = grid
            .Where(c => c.Cell == 'X')
            .SelectMany(c => Enum
                .GetValues<Direction>()
                .Select(d => new Match(c.X, c.Y, d, 1)))
            .ToList();
        var nextStates = new List<Match>();

        var total = 0;

        while (states.Count > 0)
        {
            foreach (var state in states)
            {
                var (fromX, fromY, direction, found) = state;

                if (found == 4)
                {
                    total += 1;
                    continue;
                }

                var (x, y) = direction switch
                {
                    Direction.N => (fromX, fromY - found),
                    Direction.Ne => (fromX + found, fromY - found),
                    Direction.E => (fromX + found, fromY),
                    Direction.Se => (fromX + found, fromY + found),
                    Direction.S => (fromX, fromY + found),
                    Direction.Sw => (fromX - found, fromY + found),
                    Direction.W => (fromX - found, fromY),
                    Direction.Nw => (fromX - found, fromY - found),
                    _ => throw new ArgumentOutOfRangeException(nameof(direction), direction, null)
                };

                if (x < 0 || x >= grid.Width || y < 0 || y >= grid.Height)
                    continue;

                var next = found switch
                {
                    1 => 'M',
                    2 => 'A',
                    3 => 'S',
                    _ => throw new InvalidOperationException("Already done!")
                };

                if (grid[x, y] != next)
                    continue;

                nextStates.Add(new(fromX, fromY, direction, found + 1));
            }

            states = nextStates;
            nextStates = new();
        }

        Console.WriteLine(total);
        return Task.CompletedTask;
    }
}

file sealed record Match(
    int X,
    int Y,
    Direction Direction,
    int Found);


file enum Direction
{
    N,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Nw
}

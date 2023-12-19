namespace AoC.Y2023.Solutions;

public class D17P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var grid = Grid.New(
            context.InputLines[0].Length,
            context.InputLines.Length,
            double.PositiveInfinity);

        for (var y = 0; y < grid.Height; y++)
        for (var x = 0; x < grid.Width; x++)
            grid[x, y] = context.InputLines[y][x] - '0';

        var start = Point.New(0, 0, new Movement(default, 0));
        var goal = Point.New(grid.Width - 1, grid.Height - 1, new Movement(default, 0));

        var path = grid
            .FindPathAStar(
                start,
                goal,
                (p, _) => grid.Width + grid.Height - 2 - p.X - p.Y,
                (p, comeFrom) =>
                {
                    var availableDirections = Enum
                        .GetValues<Direction>()
                        .ToList();

                    if (p.Metadata.Steps == 3)
                        availableDirections.Remove(p.Metadata.Direction!.Value);

                    var neighbors = new List<Point<Movement>>();

                    foreach (var direction in availableDirections)
                    {
                        var (x, y) = direction switch
                        {
                            Direction.Up => (p.X, p.Y - 1),
                            Direction.Down => (p.X, p.Y + 1),
                            Direction.Left => (p.X - 1, p.Y),
                            Direction.Right => (p.X + 1, p.Y),
                            _ => throw new ArgumentOutOfRangeException(nameof(direction))
                        };

                        if (x < 0 || x >= grid.Width || y < 0 || y >= grid.Height)
                            continue;
                        if (comeFrom.TryGetValue(p, out var prev) && prev.X == x && prev.Y == y)
                            continue;

                        if (x == goal.X && y == goal.Y)
                        {
                            neighbors.Add(goal);
                            continue;
                        }

                        var m = new Movement(
                            direction,
                            direction == p.Metadata.Direction
                                ? p.Metadata.Steps + 1
                                : 1);
                        neighbors.Add(Point.New(x, y, m));
                    }

                    return neighbors;
                })
            .ToList();

        var distance = path
            .Except(new[] { start })
            .Select(p => grid[p.X, p.Y])
            .Sum();

        Console.WriteLine(distance);
        return Task.CompletedTask;
    }

    private record Movement(
        Direction? Direction,
        int Steps);

    private enum Direction
    {
        Up,
        Down,
        Left,
        Right
    }
}

namespace AoC.Y2023.Solutions;

public class D17P2 : ISolution
{
    private static readonly Dictionary<Direction, Direction> _opposites = new()
    {
        [Direction.Up] = Direction.Down,
        [Direction.Down] = Direction.Up,
        [Direction.Left] = Direction.Right,
        [Direction.Right] = Direction.Left
    };

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
                (p, _) =>
                {
                    var availableDirections = GetAvailableDirections(grid, p);
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

                        if (x == goal.X && y == goal.Y)
                        {
                            if (direction == p.Metadata.Direction &&
                                p.Metadata.Steps is >= 3 and < 10)
                            {
                                neighbors.Add(goal);
                            }
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
            .Tap(x => Console.WriteLine($"{x.Metadata.Direction}({x.Metadata.Steps})"))
            .Except(new[] { start })
            .Select(p => grid[p.X, p.Y])
            .Sum();

        var displayGrid = Grid.New(
            grid.Width,
            grid.Height,
            '.');
        foreach (var p in path)
            displayGrid[p.X, p.Y] = '#';
        displayGrid.Print();

        Console.WriteLine(distance);
        return Task.CompletedTask;
    }

    private static IEnumerable<Direction> GetAvailableDirections(
        Grid<double> grid,
        Point<Movement> point)
    {
        var (x, y, (direction, steps)) = point;

        var directions = Enum
            .GetValues<Direction>()
            .AsEnumerable();

        if (x == 0)
            directions = directions.Except(new[] { Direction.Left });
        else if (x == grid.Width - 1)
            directions = directions.Except(new[] { Direction.Right });

        if (y == 0)
            directions = directions.Except(new[] { Direction.Up });
        else if (y == grid.Height - 1)
            directions = directions.Except(new[] { Direction.Down });

        if (direction is null)
            return directions;

        return steps switch
        {
            < 4 => new[] { direction.Value },
            < 10 => directions.Except(new[] { _opposites[direction.Value] }),
            _ => directions.Except(new[] { direction.Value, _opposites[direction.Value] })
        };
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

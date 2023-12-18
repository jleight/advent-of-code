namespace AoC.Y2023.Solutions;

public class D16P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var max = GenerateInitialPhotons(
                context.InputLines[0].Length,
                context.InputLines.Length)
            .Select(p => CountEnergized(context, p))
            .Max();

        Console.WriteLine(max);
        return Task.CompletedTask;
    }

    private static IEnumerable<Photon> GenerateInitialPhotons(
        int width,
        int height)
    {
        for (var i = 0; i < height; i++)
            yield return new(-1, i, Direction.Right);
        for (var i = 0; i < width; i++)
            yield return new(i, height, Direction.Up);
        for (var i = height - 1; i > 0; i--)
            yield return new(width, i, Direction.Left);
        for (var i = width - 1; i > 0; i--)
            yield return new(i, -1, Direction.Down);
    }

    private static int CountEnergized(
        SolutionContext context,
        Photon initial)
    {
        var grid = Grid.New(
            context.InputLines[0].Length,
            context.InputLines.Length,
            '.');

        var states = new List<Photon> { initial };
        var nextStates = new List<Photon>();
        var duplicates = new HashSet<Photon>();

        while (states.Count > 0)
        {
            foreach (var state in states)
            {
                if (!duplicates.Add(state))
                    continue;

                var (fromX, fromY, direction) = state;

                var (x, y) = direction switch
                {
                    Direction.Up => (fromX, fromY - 1),
                    Direction.Down => (fromX, fromY + 1),
                    Direction.Left => (fromX - 1, fromY),
                    Direction.Right => (fromX + 1, fromY),
                    _ => throw new ArgumentOutOfRangeException(nameof(direction), direction, null)
                };

                if (x < 0 || x >= grid.Width || y < 0 || y >= grid.Height)
                    continue;

                grid[x, y] = '#';

                var tile = context.InputLines[y][x];

                switch (tile)
                {
                    case '.':
                    case '|' when direction is Direction.Up or Direction.Down:
                    case '-' when direction is Direction.Left or Direction.Right:
                        nextStates.Add(new(x, y, direction));
                        break;

                    case '|':
                        nextStates.Add(new(x, y, Direction.Up));
                        nextStates.Add(new(x, y, Direction.Down));
                        break;

                    case '-':
                        nextStates.Add(new(x, y, Direction.Left));
                        nextStates.Add(new(x, y, Direction.Right));
                        break;

                    case '/' when direction == Direction.Up:
                        nextStates.Add(new(x, y, Direction.Right));
                        break;
                    case '/' when direction == Direction.Down:
                        nextStates.Add(new(x, y, Direction.Left));
                        break;
                    case '/' when direction == Direction.Left:
                        nextStates.Add(new(x, y, Direction.Down));
                        break;
                    case '/':
                        nextStates.Add(new(x, y, Direction.Up));
                        break;

                    case '\\' when direction == Direction.Up:
                        nextStates.Add(new(x, y, Direction.Left));
                        break;
                    case '\\' when direction == Direction.Down:
                        nextStates.Add(new(x, y, Direction.Right));
                        break;
                    case '\\' when direction == Direction.Left:
                        nextStates.Add(new(x, y, Direction.Up));
                        break;
                    case '\\':
                        nextStates.Add(new(x, y, Direction.Down));
                        break;

                    default:
                        throw new InvalidOperationException($"Invalid tile or direction: {direction} on '{context.InputLines[fromY][fromX]}'");
                }

            }

            states = nextStates;
            nextStates = new();
        }

        return grid.Count;
    }

    private record Photon(
        int FromX,
        int FromY,
        Direction Direction);

    private enum Direction
    {
        Up,
        Down,
        Left,
        Right
    }
}

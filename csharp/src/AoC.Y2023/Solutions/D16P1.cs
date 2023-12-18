namespace AoC.Y2023.Solutions;

public class D16P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var grid = Grid.New(
            context.InputLines[0].Length,
            context.InputLines.Length,
            '.');

        var states = new List<Photon> { new(-1, 0, Direction.Right) };
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

        Console.WriteLine(grid.Count);
        return Task.CompletedTask;
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

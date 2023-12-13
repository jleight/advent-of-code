namespace AoC.Y2023.Solutions;

public class D10P2 : ISolution
{
    private static readonly (int, int) _north = (0, -1);
    private static readonly (int, int) _east = (1, 0);
    private static readonly (int, int) _south = (0, 1);
    private static readonly (int, int) _west = (-1, 0);

    private static readonly Dictionary<char, (int X, int Y)[]> _possibleMoves = new()
    {
        ['|'] = new[] { _north, _south },
        ['-'] = new[] { _east, _west },
        ['L'] = new[] { _north, _east },
        ['J'] = new[] { _north, _west },
        ['7'] = new[] { _south, _west },
        ['F'] = new[] { _south, _east }
    };

    public Task Run(SolutionContext context)
    {
        var (grid, startX, startY) = PopulateGrid(context.InputLines);
        var (firstDeltaX, firstDeltaY) = FindFirstMove(grid, startX, startY);

        var pipeGrid = Grid.New(
            context.InputLines[0].Length,
            context.InputLines.Length,
            ' ');
        pipeGrid[startX, startY] = '|';

        var (prevX, prevY) = (startX, startY);
        var (x, y) = (prevX + firstDeltaX, prevY + firstDeltaY);

        while ((x, y) != (startX, startY))
        {
            var current = grid[x, y];

            pipeGrid[x, y] = current;

            var (nextX, nextY) = _possibleMoves[current]
                .Select(m => (x + m.X, y + m.Y))
                .Except(new[] {(prevX, prevY)})
                .First();

            (prevX, prevY) = (x, y);
            (x, y) = (nextX, nextY);
        }

        var enclosedCells = 0;

        for (var i = 0; i < pipeGrid.Height; i++)
        {
            var walls = 0;

            for (var j = 0; j < pipeGrid.Width; j++)
            {
                switch (pipeGrid[j, i])
                {
                    case ' ':
                        if (walls % 2 == 1)
                        {
                            enclosedCells += 1;
                            pipeGrid[j, i] = '*';
                        }
                        break;
                    case '|':
                    case 'L':
                    case 'J':
                        walls += 1;
                        break;
                }
            }
        }

        Console.WriteLine(enclosedCells);
        return Task.CompletedTask;
    }

    private static (Grid<char>, int, int) PopulateGrid(IReadOnlyList<string> input)
    {
        var grid = Grid.New(
            input[0].Length,
            input.Count,
            ' ');

        var startX = 0;
        var startY = 0;

        for (var y = 0; y < grid.Height; y++)
        for (var x = 0; x < grid.Width; x++)
        {
            grid[x, y] = input[y][x];

            if (input[y][x] != 'S')
                continue;

            startX = x;
            startY = y;
        }

        return (grid, startX, startY);
    }

    private static (int X, int Y) FindFirstMove(Grid<char> grid, int x, int y)
    {
        if (grid[x, y - 1] is '|' or '7' or 'F')
            return (0, -1);
        if (grid[x + 1, y] is '-' or 'J' or '7')
            return (1, 0);
        if (grid[x, y + 1] is '|' or 'J' or 'L')
            return (0, 1);
        if (grid[x - 1, y] is '-' or 'F' or 'L')
            return (-1, 0);
        throw new InvalidOperationException("There is no valid first move");
    }
}

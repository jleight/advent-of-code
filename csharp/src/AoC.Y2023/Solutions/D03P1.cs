namespace AoC.Y2023.Solutions;

public class D03P1 : ISolution
{
    private static readonly ICell _emptyCell = new EmptyCell();
    private static readonly ICell _symbolCell = new SymbolCell();

    public Task Run(SolutionContext context)
    {
        var height = context.InputLines.Length + 2;
        var width = context.InputLines[0].Length + 2;
        var grid = new ICell[height, width];

        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                if (y == 0 || y == height - 1 ||
                    x == 0 || x == width - 1)
                {
                    grid[y, x] = _emptyCell;
                    continue;
                }

                var rest = context.InputLines[y - 1][(x - 1)..];
                var character = rest[0];

                if (character == '.')
                {
                    grid[y, x] = _emptyCell;
                }
                else if (char.IsDigit(character))
                {
                    var numbers = rest
                        .TakeWhile(char.IsDigit)
                        .ToArray();
                    var partNumber = int.Parse(new string(numbers));

                    var cell = new PartNumberCell(x, y, partNumber);
                    for (var i = 0; i < numbers.Length; i++)
                        grid[y, x++] = cell;

                    x--;
                }
                else
                {
                    grid[y, x] = _symbolCell;
                }
            }
        }

        var adjacent = new HashSet<PartNumberCell>();

        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                if (grid[y, x] is not PartNumberCell p)
                    continue;

                if (grid[y - 1, x - 1] is SymbolCell)
                    adjacent.Add(p);
                else if (grid[y - 1, x] is SymbolCell)
                    adjacent.Add(p);
                else if (grid[y - 1, x + 1] is SymbolCell)
                    adjacent.Add(p);
                else if (grid[y, x - 1] is SymbolCell)
                    adjacent.Add(p);
                else if (grid[y, x + 1] is SymbolCell)
                    adjacent.Add(p);
                else if (grid[y + 1, x - 1] is SymbolCell)
                    adjacent.Add(p);
                else if (grid[y + 1, x] is SymbolCell)
                    adjacent.Add(p);
                else if (grid[y + 1, x + 1] is SymbolCell)
                    adjacent.Add(p);
            }
        }

        var result = adjacent
            .Sum(c => c.Number);

        Console.WriteLine(result);
        return Task.CompletedTask;
    }

    private interface ICell;
    private record EmptyCell : ICell;
    private record SymbolCell : ICell;
    private record PartNumberCell(int X, int Y, int Number) : ICell;
}

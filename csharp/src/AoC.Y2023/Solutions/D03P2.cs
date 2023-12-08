namespace AoC.Y2023.Solutions;

public class D03P2 : ISolution
{
    private static readonly ICell _emptyCell = new EmptyCell();
    private static readonly ICell _gearCell = new GearCell();

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

                if (character == '*')
                {
                    grid[y, x] = _gearCell;
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
                    grid[y, x] = _emptyCell;
                }
            }
        }

        var gearRatios = new List<int>();

        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                if (grid[y, x] is not GearCell)
                    continue;

                var adjacent = new HashSet<PartNumberCell>();

                if (grid[y - 1, x - 1] is PartNumberCell p1)
                    adjacent.Add(p1);
                if (grid[y - 1, x] is PartNumberCell p2)
                    adjacent.Add(p2);
                if (grid[y - 1, x + 1] is PartNumberCell p3)
                    adjacent.Add(p3);
                if (grid[y, x - 1] is PartNumberCell p4)
                    adjacent.Add(p4);
                if (grid[y, x + 1] is PartNumberCell p5)
                    adjacent.Add(p5);
                if (grid[y + 1, x - 1] is PartNumberCell p6)
                    adjacent.Add(p6);
                if (grid[y + 1, x] is PartNumberCell p7)
                    adjacent.Add(p7);
                if (grid[y + 1, x + 1] is PartNumberCell p8)
                    adjacent.Add(p8);

                if (adjacent.Count == 2)
                    gearRatios.Add(adjacent.Aggregate(1, (a, e) => a * e.Number));
            }
        }

        var result = gearRatios.Sum();

        Console.WriteLine(result);
        return Task.CompletedTask;
    }

    private interface ICell;
    private record EmptyCell : ICell;
    private record GearCell : ICell;
    private record PartNumberCell(int X, int Y, int Number) : ICell;
}

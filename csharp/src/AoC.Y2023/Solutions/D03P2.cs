namespace AoC.Y2023.Solutions;

public class D03P2 : ISolution
{
    private static readonly ICell _emptyCell = new EmptyCell();
    private static readonly ICell _gearCell = new GearCell();

    public Task Run(SolutionContext context)
    {

        var grid = Grid.New(
            context.InputLines[0].Length,
            context.InputLines.Length,
            _emptyCell);

        for (var y = 0; y < grid.Height; y++)
        for (var x = 0; x < grid.Width; x++)
        {
            var rest = context.InputLines[y][x..];
            var character = rest[0];

            if (character == '*')
            {
                grid[x, y] = _gearCell;
                continue;
            }

            if (!char.IsDigit(character))
                continue;

            var numbers = rest
                .TakeWhile(char.IsDigit)
                .ToArray();
            var partNumber = int.Parse(new string(numbers));

            var cell = new PartNumberCell(x, y, partNumber);
            for (var i = 0; i < numbers.Length; i++)
                grid[x++, y] = cell;

            x--;
        }

        var result = grid
            .Select((point, value) =>
            {
                if (value is not GearCell)
                    return 0;

                var (x, y) = point;
                var adjacent = new HashSet<PartNumberCell>();

                if (grid[x - 1, y - 1] is PartNumberCell p1)
                    adjacent.Add(p1);
                if (grid[x, y - 1] is PartNumberCell p2)
                    adjacent.Add(p2);
                if (grid[x + 1, y - 1] is PartNumberCell p3)
                    adjacent.Add(p3);
                if (grid[x - 1, y] is PartNumberCell p4)
                    adjacent.Add(p4);
                if (grid[x + 1, y] is PartNumberCell p5)
                    adjacent.Add(p5);
                if (grid[x - 1, y + 1] is PartNumberCell p6)
                    adjacent.Add(p6);
                if (grid[x, y + 1] is PartNumberCell p7)
                    adjacent.Add(p7);
                if (grid[x + 1, y + 1] is PartNumberCell p8)
                    adjacent.Add(p8);

                return adjacent.Count == 2
                    ? adjacent.Aggregate(1, (a, e) => a * e.Number)
                    : 0;
            })
            .Sum();

        Console.WriteLine(result);
        return Task.CompletedTask;
    }

    private interface ICell;
    private record EmptyCell : ICell;
    private record GearCell : ICell;
    private record PartNumberCell(int X, int Y, int Number) : ICell;
}

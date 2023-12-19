namespace AoC.Y2023.Solutions;

public class D03P1 : ISolution
{
    private static readonly ICell _emptyCell = new EmptyCell();
    private static readonly ICell _symbolCell = new SymbolCell();

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

            if (character == '.')
                continue;

            if (!char.IsDigit(character))
            {
                grid[x, y] = _symbolCell;
                continue;
            }

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
            .Select(cell =>
            {
                var (x, y, value) = cell;

                if (value is not PartNumberCell p)
                    return default;

                if (grid[x - 1, y - 1] is SymbolCell)
                    return p;
                if (grid[x, y - 1] is SymbolCell)
                    return p;
                if (grid[x + 1, y - 1] is SymbolCell)
                    return p;
                if (grid[x - 1, y] is SymbolCell)
                    return p;
                if (grid[x + 1, y] is SymbolCell)
                    return p;
                if (grid[x - 1, y + 1] is SymbolCell)
                    return p;
                if (grid[x, y + 1] is SymbolCell)
                    return p;
                if (grid[x + 1, y + 1] is SymbolCell)
                    return p;

                return default;
            })
            .OfType<PartNumberCell>()
            .ToHashSet()
            .Sum(p => p.Number);

        Console.WriteLine(result);
        return Task.CompletedTask;
    }

    private interface ICell;
    private record EmptyCell : ICell;
    private record SymbolCell : ICell;
    private record PartNumberCell(int X, int Y, int Number) : ICell;
}

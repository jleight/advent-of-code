namespace AoC.Y2022.Solutions;

[UsedImplicitly]
public class D08P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var rows = context.InputLines.Length;
        var columns = context.InputLines[0].Length;

        var results = new bool[rows, columns];
        var result = 0;

        for (var r = 0; r < rows; r++)
        {
            var maxHeight = -1;

            for (var c = 0; c < columns; c++)
            {
                var height = context.InputLines[r][c] - '0';
                results[r, c] = height > maxHeight;
                maxHeight = Math.Max(maxHeight, height);
            }
        }

        for (var c = 0; c < columns; c++)
        {
            var maxHeight = -1;

            for (var r = 0; r < rows; r++)
            {
                var height = context.InputLines[r][c] - '0';
                results[r, c] |= height > maxHeight;
                maxHeight = Math.Max(maxHeight, height);
            }
        }

        for (var r = 0; r < rows; r++)
        {
            var maxHeight = -1;

            for (var c = columns - 1; c >= 0; c--)
            {
                var height = context.InputLines[r][c] - '0';
                results[r, c] |= height > maxHeight;
                maxHeight = Math.Max(maxHeight, height);
            }
        }

        for (var c = 0; c < columns; c++)
        {
            var maxHeight = -1;

            for (var r = rows - 1; r >= 0; r--)
            {
                var height = context.InputLines[r][c] - '0';
                results[r, c] |= height > maxHeight;
                maxHeight = Math.Max(maxHeight, height);

                if (results[r, c])
                    result += 1;
            }
        }

        Console.WriteLine(result);
        return Task.CompletedTask;
    }
}

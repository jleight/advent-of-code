namespace AoC.Y2022.Solutions;

[UsedImplicitly]
public class D08P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var rows = context.InputLines.Length;
        var columns = context.InputLines[0].Length;

        var result = 0;

        for (var r = 1; r < rows - 1; r++)
        {
            for (var c = 1; c < columns - 1; c++)
            {
                var height = context.InputLines[r][c] - '0';

                var dn = 1;
                while (r - dn > 0 && height > context.InputLines[r - dn][c] - '0')
                    dn += 1;

                var de = 1;
                while (c + de < columns - 1 && height > context.InputLines[r][c + de] - '0')
                    de += 1;

                var ds = 1;
                while (r + ds < rows - 1 && height > context.InputLines[r + ds][c] - '0')
                    ds += 1;

                var dw = 1;
                while (c - dw > 0 && height > context.InputLines[r][c - dw] - '0')
                    dw += 1;

                result = Math.Max(result, dn * de * ds * dw);
            }
        }

        Console.WriteLine(result);
        return Task.CompletedTask;
    }
}

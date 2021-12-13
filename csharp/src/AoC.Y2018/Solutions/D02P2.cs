namespace AoC.Y2018.Solutions;

[UsedImplicitly]
public class D02P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var lines = context.InputLines;

        for (var ai = 0; ai < lines.Length; ai++)
        {
            var a = lines[ai];

            for (var bi = ai + 1; bi < lines.Length; bi++)
            {
                var b = lines[bi];

                var diffs = 0;
                for (var i = 0; i < a.Length; i++)
                {
                    diffs += a[i] == b[i] ? 0 : 1;
                    if (diffs > 1)
                        continue;
                }
                if (diffs != 1)
                    continue;

                for (var i = 0; i < a.Length; i++)
                {
                    if (a[i] == b[i])
                        Console.Write(a[i]);
                }
                Console.WriteLine();
            }
        }

        Console.WriteLine("No off-by-ones!");
        return Task.CompletedTask;
    }
}

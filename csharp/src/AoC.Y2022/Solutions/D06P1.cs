namespace AoC.Y2022.Solutions;

public class D06P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        foreach (var line in context.InputLines)
        {
            for (var i = 3; i < line.Length; i++)
            {
                var a = line[i - 3];
                var b = line[i - 2];
                var c = line[i - 1];
                var d = line[i];

                if (a == b || a == c || a == d ||
                    b == c || b == d ||
                    c == d)
                {
                    continue;
                }

                Console.WriteLine(i + 1);
                break;
            }
        }

        return Task.CompletedTask;
    }
}

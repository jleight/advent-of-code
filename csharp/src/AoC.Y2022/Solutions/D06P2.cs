namespace AoC.Y2022.Solutions;

public class D06P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        foreach (var line in context.InputLines)
        {
            for (var i = 14; i < line.Length; i++)
            {
                if (line[(i - 14)..i].ToHashSet().Count != 14)
                    continue;

                Console.WriteLine(i);
                break;
            }
        }

        return Task.CompletedTask;
    }
}

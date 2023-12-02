namespace AoC.Y2022.Solutions;

public class D03P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var sum = 0;

        foreach (var group in context.InputLines.Chunk(3))
        {
            var duplicate = group[0].ToHashSet()
                .Intersect(group[1].ToHashSet())
                .Intersect(group[2].ToHashSet())
                .First();

            sum += char.IsLower(duplicate)
                ? duplicate - 'a' + 1
                : duplicate - 'A' + 27;
        }

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }
}

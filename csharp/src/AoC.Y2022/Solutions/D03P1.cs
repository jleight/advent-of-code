namespace AoC.Y2022.Solutions;

[UsedImplicitly]
public class D03P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var sum = 0;

        foreach (var line in context.InputLines)
        {
            var half = line.Length / 2;

            var duplicate = line[..half].ToHashSet()
                .Intersect(line[half..].ToHashSet())
                .First();

            sum += char.IsLower(duplicate)
                ? duplicate - 'a' + 1
                : duplicate - 'A' + 27;
        }

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }
}

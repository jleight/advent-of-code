namespace AoC.Y2017.Solutions;

[UsedImplicitly]
public class D01P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputString
            .Select(c => Convert.ToInt32(c.ToString(), 10))
            .ToArray();

        var a = 0;
        var b = 1;
        var sum = 0;
        while (a < input.Length)
        {
            if (input[a] == input[b % input.Length])
                sum += input[a];
            a += 1;
            b += 1;
        }

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }
}

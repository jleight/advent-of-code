namespace AoC.Y2017.Solutions;

public class D01P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputString
            .Select(c => Convert.ToInt32(c.ToString()))
            .ToArray();

        var a = 0;
        var b = input.Length / 2;
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

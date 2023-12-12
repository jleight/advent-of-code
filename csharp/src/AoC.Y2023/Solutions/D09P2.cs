namespace AoC.Y2023.Solutions;

public class D09P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var sum = 0L;

        foreach (var line in context.InputLines)
        {
            var numbers = line
                .Split(' ')
                .Select(long.Parse)
                .Reverse()
                .ToArray();

            var derivatives = new List<long[]>
            {
                numbers
            };

            while (derivatives[^1].ToHashSet().Count > 1)
                derivatives.Add(Derivative(derivatives[^1]));

            var next = 0L;
            for (var i = derivatives.Count - 1; i >= 0; i--)
                next += derivatives[i][^1];

            sum += next;
        }

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }

    private static long[] Derivative(long[] input)
    {
        var result = new long[input.Length - 1];

        for (var i = 0; i < input.Length - 1; i++)
            result[i] = input[i + 1] - input[i];

        return result;
    }
}

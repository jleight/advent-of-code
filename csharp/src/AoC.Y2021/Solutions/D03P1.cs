namespace AoC.Y2021.Solutions;

[UsedImplicitly]
public class D03P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputLines
            .Select(l => l.ToCharArray())
            .ToArray();

        var oneBits = new int[input[0].Length];
        foreach (var line in input)
            for (var i = 0; i < line.Length; i++)
                oneBits[i] += line[i] == '1' ? 1 : 0;

        var gammaBits = new char[input[0].Length];
        var epsilonBits = new char[input[0].Length];
        for (var i = 0; i < oneBits.Length; i++)
        {
            gammaBits[i] = oneBits[i] > input.Length - oneBits[i] ? '1' : '0';
            epsilonBits[i] = gammaBits[i] == '1' ? '0' : '1';
        }

        var gamma = Convert.ToInt32(new string(gammaBits), 2);
        var epsilon = Convert.ToInt32(new string(epsilonBits), 2);

        Console.WriteLine(gamma * epsilon);
        return Task.CompletedTask;
    }
}

namespace AoC.Y2023.Solutions;

public class D15P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var instructions = context.InputLines
            .SelectMany(l => l.Split(','))
            .Select(Instruction.Parse);

        var boxes = new Dictionary<int, List<Lens>>();

        foreach (var instruction in instructions)
        {
            var hash = Hash(instruction.Label);

            if (!boxes.TryGetValue(hash, out var box))
                boxes[hash] = box = new();

            var lens = box.FirstOrDefault(l => l.Label == instruction.Label);

            if (instruction.Operation == '=')
            {
                if (lens is null)
                    box.Add(lens = instruction.ToLens());
                lens.Value = instruction.Value;
            }
            else
            {
                if (lens is null)
                    continue;
                box.Remove(lens);
            }
        }

        var sum = 0L;

        foreach (var (hash, box) in boxes)
        {
            for (var i = 0; i < box.Count; i++)
                sum += (hash + 1) * (i + 1) * box[i].Value;
        }

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }

    private static int Hash(string input)
    {
        var current = 0;

        foreach (var c in input)
        {
            current += c;
            current *= 17;
            current %= 256;
        }

        return current;
    }

    private record Instruction(
        string Label,
        char Operation,
        int Value)
    {
        public static Instruction Parse(string input)
        {
            if (input.Contains('-'))
            {
                return new Instruction(
                    input[..^1],
                    '-',
                    default);
            }

            var split = input.Split('=');

            return new Instruction(
                split[0],
                '=',
                int.Parse(split[1]));
        }

        public Lens ToLens()
            => new(Label, Value);
    }

    private class Lens(
        string label,
        int value)
    {
        public string Label { get; } = label;
        public int Value { get; set; } = value;
    }
}

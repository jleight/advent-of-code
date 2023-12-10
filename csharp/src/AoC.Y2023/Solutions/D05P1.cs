namespace AoC.Y2023.Solutions;

public class D05P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var seeds = context.InputLines[0]
            .Replace("seeds: ", string.Empty)
            .Split(' ')
            .Select(long.Parse)
            .ToHashSet();

        var maps = new List<Map>();

        for (var i = 2; i < context.InputLines.Length; i++)
        {
            var header = context.InputLines[i];
            var types = header.Split(' ')[0].Split('-');
            var source = types[0];
            var destination = types[2];

            var converters = context.InputLines
                .Skip(i + 1)
                .TakeWhile(l => l.Length > 0)
                .Select(l => l.Split(' ').Select(long.Parse).ToArray())
                .Select(n => new Converter(n[1], n[1] + n[2] - 1, n[0]))
                .OrderBy(c => c.SourceStart)
                .ToList();
            i += converters.Count + 1;

            maps.Add(new Map(source, destination, converters));
        }

        var minLocation = long.MaxValue;

        foreach (var seed in seeds)
        {
            var value = seed;
            var unit = "seed";

            while (unit != "location")
            {
                var map = maps.First(m => m.Source == unit);

                value = map.Converters
                    .FirstOrDefault(c => c.SourceStart <= value && value <= c.SourceEnd)?
                    .Convert(value) ?? value;
                unit = map.Destination;
            }

            minLocation = Math.Min(minLocation, value);
        }

        Console.WriteLine(minLocation);
        return Task.CompletedTask;
    }

    private record Map(
        string Source,
        string Destination,
        List<Converter> Converters);

    private record Converter(
        long SourceStart,
        long SourceEnd,
        long DestinationStart)
    {
        public long Convert(long x)
            => DestinationStart + (x - SourceStart);
    }
}

namespace year_2022.Days;

public class Day9
{
    private List<string> _input = new();
    private IDictionary<string, int> _places = new Dictionary<string, int>();
    
    public string Part1()
    {
        ParseInput("day9-1");

        var tailPlaces = DoStuff();

        return tailPlaces.ToString();
    }

    public string Part2()
    {
        ParseInput("day9-1");
        
        var tailPlaces = DoStuff();

        return tailPlaces.ToString();
    }

    public string Parttest()
    {
        ParseInput("day9-0");

        var tailPlaces = DoStuff();

        return tailPlaces.ToString();
    }

    int DoStuff()
    {
        var rope = new[] {new Point(), new Point(), new Point(), new Point(), new Point(), new Point(), new Point(), new Point(), new Point(), new Point()};
        
        foreach (var instruction in _input)
        {
            var move = instruction.Split(" ");

            for (int i = 0; i < Int32.Parse(move[1]); i++)
            {
                switch (move[0])
                {
                    case "R":
                        rope[0].X++;
                        MoveTail(ref rope, "R");
                        break;
                    case "L":
                        rope[0].X--;
                        MoveTail(ref rope, "L");
                        break;
                    case "U":
                        rope[0].Y++;
                        MoveTail(ref rope, "U");
                        break;
                    case "D":
                        rope[0].Y--;
                        MoveTail(ref rope, "D");
                        break;
                }
            }
        }
        
        return _places.Count;
    }

    void MoveTail(ref Point[] rope, string direction)
    {
        for (int i = 1; i < rope.Length; i++)
        {
            var parent = rope[i - 1];

            if (parent.X == rope[i].X && parent.Y == rope[i].Y)
                break;
            
            // check up
            if (parent.Y > rope[i].Y + 1)
            {
                if (parent.X == rope[i].X)
                {
                    rope[i].Y++;
                } else if (parent.X > rope[i].X)
                {
                    rope[i].Y++;
                    rope[i].X++;
                } else if (parent.X < rope[i].X)
                {
                    rope[i].Y++;
                    rope[i].X--;
                }
            }
            
            // check down
            if (parent.Y < rope[i].Y - 1)
            {
                if (parent.X == rope[i].X)
                {
                    rope[i].Y--;
                } else if (parent.X > rope[i].X)
                {
                    rope[i].Y--;
                    rope[i].X++;
                } else if (parent.X < rope[i].X)
                {
                    rope[i].Y--;
                    rope[i].X--;
                }
            }
            
            // check left
            if (parent.X < rope[i].X - 1)
            {
                if (parent.Y == rope[i].Y)
                {
                    rope[i].X--;
                } else if (parent.Y > rope[i].Y)
                {
                    rope[i].Y++;
                    rope[i].X--;
                } else if (parent.Y < rope[i].Y)
                {
                    rope[i].Y--;
                    rope[i].X--;
                }
            }
            
            // check right
            if (parent.X > rope[i].X + 1)
            {
                if (parent.Y == rope[i].Y)
                {
                    rope[i].X++;
                } else if (parent.Y > rope[i].Y)
                {
                    rope[i].Y++;
                    rope[i].X++;
                } else if (parent.Y < rope[i].Y)
                {
                    rope[i].Y--;
                    rope[i].X++;
                }
            }
        }
        
        _places[rope[9].X + "," + rope[9].Y] = 1;
    }
    
    void ParseInput(string name)
    {
        foreach (string line in File.ReadLines(@"/home/clayton/Code/adventofcode/year2022/Inputs/" + name + ".txt"))
        {
            _input.Add(line);
        }  
    }
}

public struct Point 
{
    public int X {get;set;}
    public int Y {get;set;}

    public Point()
    {
        X = 0;
        Y = 0;
    }
}
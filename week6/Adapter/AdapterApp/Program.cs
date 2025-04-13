using System;

class InternalClass
{
    public string Fetch()
    {
        return "get user info";
    }
}

class ExternalClass
{
    public string Search()
    {
        return "get user info";
    }
}

class Adapter
{
    private ExternalClass external;

    public Adapter(ExternalClass ext)
    {
        this.external = ext;
    }

    public string Fetch()
    {
        return external.Search();
    }
}

class Program
{
    static void Main()
    {
        var internalObj = new InternalClass();
        Console.WriteLine("Internal: " + internalObj.Fetch());

        var external = new ExternalClass();
        var adapter = new Adapter(external);
        Console.WriteLine("External through Adapter: " + adapter.Fetch());
    }
}

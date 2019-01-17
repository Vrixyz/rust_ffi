using System;
using System.Runtime.InteropServices;

namespace c_sharp
{

    [StructLayout(LayoutKind.Sequential)]
    public struct SampleStruct
    {
        public Int32 field_one;
        public Int32 field_two;
        public IntPtr string_field;
    }
    class Program
    {
        [DllImport("libs/librust_ffi_lib.dylib")]
        private static extern SampleStruct do_stuff(Int32 number1, Int32 number2);
        [DllImport("libs/librust_ffi_lib.dylib")]
        private static extern SampleStruct do_stuff_with(SampleStruct param);
        [DllImport("libs/librust_ffi_lib.dylib")]
        private static extern SampleStruct free_string();
        static void Main(string[] args)
        {
            var simple_struct = do_stuff(2, 40);
            Console.WriteLine(simple_struct.field_one);
            Console.WriteLine(simple_struct.field_two);
            do_stuff_with(simple_struct);
            Console.WriteLine(Marshal.PtrToStringAnsi(simple_struct.string_field));
            free_string();
        }
    }
}

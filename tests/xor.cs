using System;

class Xor {
    static void Main() {
        byte[] buf = new byte[16] {0x0b,0x1a,0x29,0x38,0x4f,0x5e,
        0x6d,0x7c,0x83,0x92,0xa1,0xb0,0xc7,0xd6,0xe5,0xf4};

        for (int i = 0; i < buf.Length; i++) {
            buf[i] = (byte)(buf[i] ^ 11);
        }

        for (int i = 0; i < buf.Length; i++) {
            Console.Write(String.Format("0x{0:X2},", buf[i]));
        }
        Console.WriteLine();
    }
}
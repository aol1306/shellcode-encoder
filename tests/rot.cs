using System;

class Rot {
    static void Main() {
        byte[] buf = new byte[16] {0x0b,0x1c,0x2d,0x3e,0x4f,0x60,
        0x71,0x82,0x93,0xa4,0xb5,0xc6,0xd7,0xe8,0xf9,0x0a};

        for (int i = 0; i < buf.Length; i++) {
            buf[i] = (byte)((buf[i] - 11) & 0xff);
        }

        for (int i = 0; i < buf.Length; i++) {
            Console.Write(String.Format("0x{0:X2},", buf[i]));
        }
        Console.WriteLine();
    }
}
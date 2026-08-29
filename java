QUESTION 0
if (number>0) {
    System.out.println(number + "is positive");
} else if (number == 0 ){
     System.out.println(number + "is zero");
} else{
      System.out.printn(number + "is negative");
}

QUESTION 1
int lastDigit = number % 10;
System.out.println("Last digit of " + number + "is" + lastDigit);

if (lastDigit > 5) {
    System.out.println(" and is greater than 5");
} else if (lastDigit ==0") {
     System.out.printn(" and is 0");
} else {
      System.out.println(" and is less than 6 and not 0");
}
QUESTION 2
 for (char letter = 'a'; letter <= 'z'; letter++) {
      System.out.println(letter);
{
QUESTION 3
for (char letter = 'a'; letter <= 'z'; letter++) {
    if (letter != 'q' && letter != 'e') {
     System.out.println(letter);
    }
  }
QUESTION 4
for (int i =0; <= 98; i++) {
   System.out.println(i + "=0x + Interger.toHexString(i));
}
QUESTION 5
for (int i = 0; i <= 99; i++) {
    if(i < 99 ) {
    System.out.printf("%02d", i);
  } else {
      System.out.ptintf("%02d%n", i);
   }
}
QUESTIOLN 6 
for (int i = 0; i <= 89; i++) {
    int tens = i / 10;
    int units = i % 10;
    if (tens < units ) {
        System.out.printf("%d%d", tens, units);
     }
}
System.out.println("89");
QUESTION 7 
pubic static boolean islower(char c){
     return c >= 'a' && c <= 'z';
{
QUESTION 8
public static void uppercase(String str) {
      char[]  chars = str.toCharArray() ;
      for (int i = 0; i < chars.length; i++) {
         if (chars[i] >= 'a' && chars[i] <= 'z')  {
             chars[i] = (char) (chars [i] - 32);
     }
  }
  System.out.println( new String(chars));
}
QUESTION 9
public static int print_last_digit(int number) {
    int lastDigit = (number % 10 + 10) %10;
    System.out.println(lastDigit);
    Return lastDigit;
}
QUESTION 10
public static int add(int a ,int b) {
    return a + b;
}


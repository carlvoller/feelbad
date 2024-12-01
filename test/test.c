#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {

  char text[] = "testing\n";

  int written = write(2, text, 8);


  return 0;

}
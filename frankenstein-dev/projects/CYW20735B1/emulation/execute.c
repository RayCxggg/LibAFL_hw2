#include "common.h"

void do_exit()
{
    print("do_exit\n");
    exit(0);
}

void _start()
{
    patch_code();
    idle_loop = do_exit;
    cont();
}

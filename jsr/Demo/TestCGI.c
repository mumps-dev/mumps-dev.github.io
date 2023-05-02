#include <stdio.h>
#include <stdlib.h>

int main ()
{
#include "EdMGTM.h"

    /* MUMPS program to execute */
    char mumpspgm [] = "^TestCGI";

    char routpath [256];
    char execute [256];
    char gtmgbldir [256];

    setenv("gtm_dist", gtmdir, 1);

    sprintf(gtmgbldir, "%s%s", appdir, gtmdb);
    setenv("gtmgbldir", gtmgbldir, 1);

    sprintf (routpath, "%s %s", appdir, gtmdir);
    setenv("gtmroutines", routpath, 1);

    sprintf(execute,"cd %s;%s/mumps -run %s", appdir, gtmdir, mumpspgm);

    if (system(execute))
    { /* Error occured */
        printf("CONTENT-TYPE: TEXT/PLAIN\n\nError!!!\n");
        printf("gtmdir      = %s\n", gtmdir);
        printf("database    = %s\n", gtmdb);
        printf("gtmgbldir   = %s\n", getenv("gtmgbldir"));
        printf("gtmroutines = %s\n", getenv("gtmroutines"));
        printf("gtm_dist    = %s\n", getenv("gtm_dist"));
        printf("execute     = %s\n", execute);
    }
}            

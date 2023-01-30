#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#define DMAX 10000
#define NMAX 21
#define SMAX 10 

typedef struct 
{
    char name[NMAX];
    int size[SMAX];
    int saldo[SMAX];
}Drugs;

void regNewDrug(Drugs lakemedel[], int *nrOfDrugs)
{
    int q=1;
    while (q==1)
    {
        int i=0, nLength, same;
        char testName[NMAX];
        if (*nrOfDrugs<DMAX)
        {
            printf("Ange namn: ");
            scanf(" %s", testName);
            nLength=strlen(testName);
            if (nLength>=NMAX)
            {
                printf("For stort namn pa lakemedlet\n");
                break;
            }
            else
            {
                for (i=0;i<(*nrOfDrugs);i++)
                {
                    same=strcmp(testName,lakemedel[i].name);
                    if (same==0)
                    {
                        printf("Lakemedlet ar redan registrerat, var vanlig och valj ett nytt lakemedel\n");
                        break;
                    }
                }
                if (same!=0)
                {
                    for(i=0;i<SMAX;i++)
                    {
                        printf("Ange storlek (0 for att avsluta): ");
                        scanf("%d", &lakemedel[*nrOfDrugs].size[i]);
                        lakemedel[*nrOfDrugs].saldo[i]=0;
                        if (lakemedel[*nrOfDrugs].size[i]==0)
                        {
                            i=SMAX;
                        }
                    }
                    strcpy(lakemedel[*nrOfDrugs].name,testName);
                    (*nrOfDrugs)++;
                    q=0;
                    break;
                }
            }
        }
        else
        {
            printf("Du har uppnat maximalt antal structar\n");
            break;
        }
    }
}

void printDrugs(Drugs lakemedel[], int *nrOfDrugs)
{
    printf("Lakemedel\t Storlekar\t Saldo \n");
    printf("_________________________________________\n");
    for (int i=0;i<=(*nrOfDrugs);i++)
    {
        printf("%s\t\t  ", lakemedel[i].name);
        for(int j=0;j<=SMAX;j++) 
        {
            if (lakemedel[i].size[j]==0)
            {
                break;
            }
            else
            {
                printf("%d,", lakemedel[i].size[j]);
            }
        }
        printf("\t\t");
        for(int j=0;j<=SMAX;j++) 
        {
            if (lakemedel[i].size[j]==0)
            {
                break;
            }
            else
            {
                printf("%d,", lakemedel[i].saldo[j]);
            }
        }
        printf("\n");
    }
}

void searchDrug(Drugs lakemedel[], int *nrOfDrugs, int *match, int *plats)
{
    int searchRaknare=0;
    *match=0;
    char *common, search[DMAX];
    printf("Ange sokstrang: ");
    scanf("%s", search);
    printf("\nLakemedel\t Storlekar\t Saldo \n");
    printf("_________________________________________\n");
    for (int i=0;i<(*nrOfDrugs);i++)
    {
        common=strstr(lakemedel[i].name,search);
        if (common)
        {
            printf("%s\t\t", lakemedel[i].name);
            for (int j=0;j<SMAX;j++)
            {
                if (lakemedel[i].size[j] == 0)
                {
                    break;
                }
                else
                {
                    printf("%d,", lakemedel[i].size[j]);
                }
            }
            printf("\t\t");
            for (int j=0;j<SMAX;j++)
            {
                if (lakemedel[i].size[j] == 0)
                {
                    break;
                }
                else
                {
                    printf("%d,", lakemedel[i].saldo[j]);
                }
            }
            searchRaknare++;
            (*match)++;
            (*plats)=i;
            printf("\n");
        }
    }
    if (searchRaknare==0)
    {
        printf("\nLakemedlet innehallande det du sokte efter finns inte registrerat");
    }
    printf("\n");
}

void unregDrug(Drugs lakemedel[], int *nrOfDrugs, int *match, int *plats)
{
    int q=1;
    char unregister=0;
    while (q==1)
    {
        searchDrug(lakemedel, nrOfDrugs, &*match, &*plats);
        if (*match==1)
        {
            printf("Vill du avregistrera %s (j/n)? ", lakemedel[*plats].name);
            scanf(" %c", &unregister);
            if (unregister=='j')
            {
                printf("%s avregistreras \n", lakemedel[*plats].name);
                for (int i=*plats;i<*nrOfDrugs;i++)
                {
                    lakemedel[i]=lakemedel[i+1];
                }
                (*nrOfDrugs)--;
                q=0;
            }
            else if (unregister=='n')
            {
                break;
            }
            else
            {
                printf("Var vanlig och skriv in antingen 'j' eller 'n' \n");
            }
        }
        else
        {
            printf("Du fick inte endast ett alternativ. Vanligen gor en ny sokning\n");
        }
    }
}

void writeToFile(Drugs lakemedel[], int nrOfDrugs, char fileName[])
{
    FILE *filePrint = fopen(fileName, "w");
    if (filePrint==NULL)
    {
        printf("\nProblem att ladda till filen\n");
    }
    fprintf(filePrint, "%d", nrOfDrugs);
    for (int i=0;i<nrOfDrugs;i++)
    {
        int j=0, storlekar=0;
        fprintf(filePrint, "\n%-30s\t", lakemedel[i].name);
        do
        {
            fprintf(filePrint, "%d ", lakemedel[i].size[j]);
            j++;
            storlekar++;
        } 
        while (lakemedel[i].size[j]!= 0);
        fprintf(filePrint, "\t\t\t\t0\t\t\t\t");
        for (int k=0;k<storlekar;k++)
        {
            fprintf(filePrint, "%d ", lakemedel[i].saldo[k]);
        }
    }
    fclose(filePrint);
}

void readFromFile(Drugs lakemedel[], int *nrOfDrugs, char fileName[])
{
    FILE *filePrint;
    printf("Ange namn pa befintlig fil eller skapa en ny: ");
    scanf(" %s", fileName);
    fileName = strcat(fileName, ".txt");
    filePrint = fopen(fileName, "r");
    if (filePrint==NULL)
    {
        printf("Filen existerar inte\nSkapar fil: %s \n", fileName);
    }
    else
    {
        fscanf(filePrint, "%d", &*nrOfDrugs);
        for (int i=0;i<*nrOfDrugs;i++)
        {
            int storlekar=0;
            fscanf(filePrint, "%s", lakemedel[i].name);
            for (int j=0;j<SMAX;j++)
            {
                fscanf(filePrint, "%d", &lakemedel[i].size[j]);
                storlekar++;
                if (lakemedel[i].size[j]==0)
                {
                    j=SMAX;
                }
            }
            for (int k=0;k<storlekar;k++)
            {
                fscanf(filePrint, "%d", &lakemedel[i].saldo[k]);
            }
        }
        fclose(filePrint);
    }
}

void databaseManagement(Drugs lakemedel[], int nrOfDrugs, char fileName[])
{
    int meny, match=0, plats; 
    while (meny!=10)
    {
        printf("Vad vill du gora?\n");
        printf("1. Registrera nytt lakemedel\n");
        printf("2. Skriva ut alla lakemedel\n");
        printf("3. Soka efter lakemedel\n");
        printf("4. Avregistrera lakemedel\n");
        printf("5. Avsluta programmet\n");
        scanf("%d", &meny);
        switch (meny)
        {
            case 1: printf("Registrera lakemedel\n");
            regNewDrug(lakemedel, &nrOfDrugs);
            break;
            case 2: printf("Skriv ut lakemedel\n");
            printDrugs(lakemedel, &nrOfDrugs);
            break;
            case 3: printf("Sok lakemedel\n");
            searchDrug(lakemedel, &nrOfDrugs, &match, &plats);
            break;
            case 4: printf("Avregistrera lakemedel\n");
            unregDrug(lakemedel, &nrOfDrugs, &match, &plats);
            break;
            case 5: printf("Avslutar programmet\n");
            writeToFile(lakemedel, nrOfDrugs, fileName);
            break;
            default: printf("Ej existerande val, skriv in siffra 1-10\n");
            break;
        }
    }   
}

int main(void)
{
    int nrOfDrugs=0;
    char fileName[NMAX];
    Drugs lakemedel[DMAX];
    readFromFile(lakemedel, &nrOfDrugs, fileName);
    databaseManagement(lakemedel, nrOfDrugs, fileName);
    return 0;
}

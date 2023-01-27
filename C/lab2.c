#include <stdio.h>
#include <math.h>
#define LENGTH 10

void view(int data[], int nrOfMeasurements);
int enter(int data[], int nrOfMeasurements);
void compute(int data[], int nrOfMeasurement);
int maxValue(int data[], int nrOfMeasurements);
int minValue(int data[], int nrOfMeasurements);
float avrValue(int data[], int nrOfMeasurements);
void normalized(int data[], int tmp[], int nrOfMeasurements, float average);

int main()
{
    char menuChoice;
    int nrOfMeasurements = 0, continueMenu = 1;
    int data[LENGTH];

    printf("Measurement tool 2.0\n");

    while(continueMenu)
    {
        printf("VECRQ? ");
        scanf(" %c", &menuChoice);
        if(menuChoice == 'v')
        {
            view(data, nrOfMeasurements);
        }
        else if(menuChoice == 'e')
        {
            nrOfMeasurements = enter(data, nrOfMeasurements);
        }
        else if(menuChoice == 'c')
        {
            compute(data, nrOfMeasurements);
        }
        else if(menuChoice == 'r')
        {
            nrOfMeasurements = 0;
        }
        else if(menuChoice == 'q')
        {
            printf("Exit measurement tool\n");
            continueMenu = 0;
        }
        else
        {
            printf("Invalid input\n");
        }
    }
    return 0;
}

int enter(int data[], int nrOfMeasurements)
{
    int oneMeasurement;
    
    for(nrOfMeasurements; nrOfMeasurements<LENGTH; nrOfMeasurements++)
    {
        printf("Enter measurement #%d (or q to quit): ",nrOfMeasurements+1);
        int readInteger = scanf("%d",&oneMeasurement);
        if(readInteger)
        {
            data[nrOfMeasurements] = oneMeasurement;
        }
        else
        {
            char tmp;
            scanf(" %c",&tmp);
            break;
        } 
    }
    return nrOfMeasurements;
}

void view(int data[], int nrOfMeasurements)
{
    int i;

    if(nrOfMeasurements > 0)
    {
    printf("[");

    for(i=0; i<nrOfMeasurements;i++)
    {
        printf(" %d ", data[i]);
    }

    printf("]\n");
    }
    else
    {
        printf("No existing measurements\n");
    }
}

void compute(int data[], int nrOfMeasurements)
{
    if(nrOfMeasurements > 0)
    {
    printf("Max value: %d\n", maxValue(data, nrOfMeasurements));

    printf("Min value: %d\n", minValue(data, nrOfMeasurements));

    printf("Avarage value: %.2f\n", avrValue(data, nrOfMeasurements));

    int i;
    int tmp[LENGTH];
    float average = avrValue(data, nrOfMeasurements);

    normalized(data, tmp, nrOfMeasurements, average);
    printf("[ ");
    for(i=0; i<nrOfMeasurements; i++)
    {
        printf("%d ", tmp[i]);
    }
    printf("]\n");
    }
    else
    {
        printf("No existing measurements\n");
    }
    
}

int maxValue(int data[], int nrOfMeasurements)
{
    int i,max=data[0];
    for(i=0;i<nrOfMeasurements;i++)
    {
        if(max<data[i])
        {
            max = data[i];
        }
    }
    return max;
}

int minValue(int data[], int nrOfMeasurements)
{
    int i,min=data[0];
    for(i=0;i<nrOfMeasurements;i++)
    {
        if(min>data[i])
        {
            min = data[i];
        } 
    }
    return min;
}

float avrValue(int data[], int nrOfMeasurements)
{
    int i; 
    float avrValue, sum = 0;

    for(i=0; i<nrOfMeasurements; i++)
    {
        sum+=data[i];
    }
    avrValue = sum/nrOfMeasurements;

    return avrValue;
}

void normalized(int data[], int tmp[], int nrOfMeasurements, float average)
{
    int i;
    int averageRounded;
    averageRounded = round(average);
    for(i=0; i<nrOfMeasurements; i++)
    {
    tmp[i] = data[i];
    }

    for(i=0; i<nrOfMeasurements; i++)
    {
    tmp[i] = tmp[i] - averageRounded;
    }
}
#include <stdio.h>

float update_height(float velocity, float height, int throttle) {
    return height + velocity + (0.1 * throttle - 1.5) / 2;
}

float update_velocity(float velocity, int throttle) {
    return velocity + (0.1 * throttle - 1.5);
}

int main(void)
{
    int t=0, throttle;
    float velocity=-25, height=250;
    printf("Lunar decent challenge!\nYou will pilot a lunar decent the last 250m.\nEach turn represent 1-second decent time.\n");
    printf("Set the throttle for each turn (0-100)\nTime Height Velocity Throttle?\n");

    while(height>=0)
    {
        printf("  %d   %.1f  %.1f   ", t, height, velocity);
        t+=1;
        scanf("%d", &throttle);
        if(throttle<0)
        {
            throttle = 0;
        }
        else if(throttle>100)
        {
            throttle = 100;
        }
        
        height = update_height(velocity, height, throttle);
        velocity = update_velocity(velocity, throttle);
        if(height<=0)
        {
            break;
        }
    }

    if(velocity<-2)
    {
        printf("FAILED! Crash landing at %.1f m/s", velocity);
    }
    else
    {
        printf("SUCCESS! Safe landing at %.1f m/s", velocity);
    }
    return 0;
}

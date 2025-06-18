#include <iostream>
#include <vector>
#include <array>

using namespace std;

vector<array<int,3>> prohibited;
int n,p,m;
long long dp[32][32],prev_state[32][32];
bool accepted_masks[32][32][32];




bool pattern_inside(array<int,3> masks, array<int,3> pattern) {
    for (int i=0;i<=2;i++) {
        if (array{masks[0]%8,masks[1]%8,masks[2]%8} == pattern) return true;
        masks = array{masks[0]/2,masks[1]/2,masks[2]/2};
    }
    return false;
}

bool check_masks(array<int,3> masks) {
    for (auto pattern : prohibited) {
        if (pattern_inside(masks, pattern)) {return false;}
    }
    return true;
}

void shift_state() {
    for (int mask1 = 0; mask1 < (1<<5); mask1++) {
        for (int mask2 = 0; mask2 < (1<<5); mask2++) {
            for (int mask3 = 0; mask3 < (1<<5); mask3++) {
                if (accepted_masks[mask1][mask2][mask3]) {
                    dp[mask2][mask3] += prev_state[mask1][mask2];
                    dp[mask2][mask3] %= m;
                }
            }
        }
    }
    for (int mask1 = 0; mask1 < (1<<5); mask1++) {
        for (int mask2 = 0; mask2 < (1<<5); mask2++) {
            prev_state[mask1][mask2] = dp[mask1][mask2];
            dp[mask1][mask2] = 0;
        }
    }
}

int main() {
    cin>>n>>p>>m;
    for (int i=0;i<p;i++) {
        int pow = 1;
        array<int,3> pattern{};
        for (int a=0;a<3;a++){
            string row;
            cin>>row;
            for (int b=0;b<3;b++) {
                if (row[b] != '.')pattern[b]+=pow;
            }
            pow*=2;
        }
       prohibited.push_back(pattern);
    }
    for (int i=0;i<(1<<5);i++)
        for (int j=0;j<(1<<5);j++) {
            prev_state[i][j] = 1;
            for (int l=0;l<(1<<5);l++) {
                accepted_masks[i][j][l] = check_masks(array{i,j,l});
            }
        }
    for (int i=3;i<=n;i++) {
        shift_state();
    }
    long long ans = 0;
    for (int i = 0;i<(1<<5);i++) {
        for (int j=0;j<(1<<5);j++) {
            ans+= prev_state[i][j];
            ans%=m;
        }
    }
    cout<<ans;
    return 0;
}

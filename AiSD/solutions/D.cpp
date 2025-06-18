#include <iostream>
#include <algorithm>
#include<vector>
using namespace std;

typedef pair<int,int> Point;
#define X first
#define Y second
pair<Point,Point> curr_result = {{-1e7,-1e7},{1e7,1e7}};

vector<Point> pot_y;
long long res = 1e16;
Point t[1000001];
int n;

long long dist(Point a, Point b) {
    long long dx = abs(a.X-b.X);
    long long dy = abs(a.Y-b.Y);
    return dx*dx + dy*dy;
}
bool cmp_x(Point a, Point b) {
    if (a.X < b.X) return true;
    if (a.X == b.X && a.Y < b.Y)return true;
    return false;
}
bool cmp_y(Point a, Point b) {
    if (a.Y < b.Y) return true;
    if (a.Y == b.Y && a.X < b.X)return true;
    return false;
}



pair<vector<Point>,vector<Point>> partition(vector<Point> sorted_by_y, int mid_point) {
    vector<Point> lef_side= {}, rig_side = {};
    for (Point p : sorted_by_y) {
        if (p.X <= mid_point) lef_side.push_back(p);
        else rig_side.push_back(p);
    }
    return {lef_side,rig_side};
}

void brut_find(int lef, int rig) {
    for (int i = lef; i < rig; i++) {
        for (int j = i+1; j < rig; j++) {
            if (dist(t[i],t[j]) < res) {
                curr_result = {t[i],t[j]};
                res = dist(t[i],t[j]);
            }
        }
    }
}
void check_potential_y(vector<Point> tab) {
    int len = tab.size();
    for (int i=0; i < len; i++) {
        for (int j = i+1; j < min(len, i+8); j++) {
            if (dist(tab[i],tab[j]) < res) {
                curr_result = {tab[i],tab[j]};
                res = dist(tab[i],tab[j]);
            }
        }
    }
}

void find_min(int lef, int rig, vector<Point> sorted_by_y) {
    if (rig - lef < 16)brut_find(lef,rig);
    else {
        const int mid = lef + (rig - lef) / 2;
        const int mid_point = t[mid].X;
        auto [lef_side,rig_side] = partition(sorted_by_y,mid_point);
        find_min(lef, mid, lef_side);
        find_min(mid, rig, rig_side);
        pot_y = {};
        for (Point p:sorted_by_y) {
            if (abs(p.X - mid_point) * abs(p.X - mid_point) < res)pot_y.push_back(p);
        }
        check_potential_y(pot_y);
    }
}





int main() {
    ios_base::sync_with_stdio(false);
    cin>>n;
    for (int i=0;i<n;i++) {
        int x,y; cin>>x>>y;
        t[i] = {x,y};
    }
    vector<Point> sorted_by_y;
    sort(t,t+n, cmp_y);
    for (int i=0;i<n;i++)sorted_by_y.push_back(t[i]);

    sort(t,t+n,cmp_x);
    find_min(0,n,sorted_by_y);
    cout<<curr_result.first.X<<" "<<curr_result.first.Y<<endl;
    cout<<curr_result.second.X<<" "<<curr_result.second.Y;

    return 0;
}
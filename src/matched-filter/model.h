#pragma once

#include <util/common/geom/point.h>
#include <util/common/math/vec.h>
#include <util/common/plot/plot.h>
#include <util/common/math/fuzzy.h>

#include <cstdint>
#include <vector>
#include <map>
#include <set>
#include <array>

#include <omp.h>

namespace model
{
    /*****************************************************/
    /*                     params                        */
    /*****************************************************/

    struct parameters
    {
        int n;
        double bitrate;
        double sampling_rate;
        double snr;
        double snr_from, snr_to; int snr_count;
        int num_of_tests;
    };

    inline parameters make_default_parameters()
    {
        parameters p =
        {
            32,
            9600,
            250000,
            -10,
            10, -10, 100,
            10
        };
        return p;
    }

    /*****************************************************/
    /*                     data                          */
    /*****************************************************/

    /*****************************************************/
    /*                     drawing                       */
    /*****************************************************/

    using points_t = std::vector < geom::point2d_t >;

    struct plot_data
    {
        util::ptr_t < points_t > data;
        plot::list_drawable < points_t > ::ptr_t plot;
    };

    template < size_t N >
    struct plot_group_data
    {
        plot::auto_viewport < points_t > ::ptr_t autoworld;
        std::array<plot_data, N> plots;
    };

    struct model_data
    {
        util::ptr_t < parameters > params;

        plot_group_data<1> i;
        plot_group_data<1> q;
        plot_group_data<4> f;
        plot_group_data<1> results;
    };

    struct demo_results
    {
        double err;
    };

    inline static plot_data make_plot_data
    (
        plot::palette::pen_ptr pen = plot::palette::pen(0xffffff),
        plot::list_data_format data_format = plot::list_data_format::chain
    )
    {
        plot_data pd;
        pd.data = util::create < points_t >();
        pd.plot = plot::list_drawable < points_t > ::create
        (
            plot::make_data_source(pd.data),
            nullptr, // no point painter
            pen
        );
        pd.plot->data_format = data_format;
        return pd;
    }

    template < size_t N >
    inline static plot_group_data<N> make_plot_group_data
    (
        std::array<plot::palette::pen_ptr, N> pens,
        plot::list_data_format data_format = plot::list_data_format::chain
    )
    {
        plot_group_data<N> pd;
        pd.autoworld = plot::min_max_auto_viewport < points_t > ::create();
        for (size_t n = 0; n < N; ++n) {
            pd.plots[n] = make_plot_data(pens[n], data_format);
        }
        return pd;
    }

    template < size_t N >
    inline static plot::drawable::ptr_t make_root_drawable
    (
        const plot_group_data<N>& p,
        std::vector < plot::drawable::ptr_t > layers
    )
    {
        using namespace plot;

        return viewporter::create(
            tick_drawable::create(
                layer_drawable::create(layers),
                const_n_tick_factory<axe::x>::create(
                    make_simple_tick_formatter(1, 5),
                    0,
                    5
                ),
                const_n_tick_factory<axe::y>::create(
                    make_simple_tick_formatter(1, 5),
                    0,
                    5
                ),
                palette::pen(RGB(150, 150, 150)),
                RGB(200, 200, 200)
            ),
            make_viewport_mapper(make_world_mapper < points_t >(p.autoworld))
        );
    }

    inline model_data make_model_data(const parameters& p = make_default_parameters())
    {
        model_data md;
        md.params = util::create < parameters >(p);
        md.i = make_plot_group_data<1>({ { plot::palette::pen() } });
        md.q = make_plot_group_data<1>({ { plot::palette::pen() } });
        md.f = make_plot_group_data<4>({ {
                plot::palette::pen(0x0000ff),
                plot::palette::pen(0x00ff00),
                plot::palette::pen(0xff0000),
                plot::palette::pen(0x000000),
        } });
        md.results = make_plot_group_data<1>({ { plot::palette::pen() } });
        return md;
    }
}
